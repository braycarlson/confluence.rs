use core::fmt;
use core::time::Duration;
use std::io::{Seek, SeekFrom, Write};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

use crate::credentials::Credentials;
use crate::error::Error;
use crate::kind::{Binary, Kind};
use crate::limits;
use crate::operation::Operation;
use crate::paged::Paged;
use crate::prng::Prng;
use crate::reply::Reply;
use crate::request::Request;
use crate::response::Response;
use crate::size;
use crate::transport::Transport;
use crate::wait::Wait;
use crate::walk::{Cursor, Walk};

const PACE_MILLIS_NEAR_LIMIT: u64 = 250;
const RETRY_AFTER_SECONDS_MAX: u64 = 900;
const RETRY_COUNT_MAX: u32 = 4;
const RETRY_DELAY_SECONDS_BASE: u64 = 5;
const RETRY_DELAY_SECONDS_MAX: u64 = 30;
const RETRY_JITTER_PERCENT_MAX: u64 = 130;
const RETRY_JITTER_PERCENT_MIN: u64 = 70;
const STATUS_MAX: u16 = 599;
const STATUS_MIN: u16 = 100;
const STATUS_SEE_OTHER: u16 = 303;
const STATUS_SERVER_ERROR_MIN: u16 = 500;
const STATUS_SUCCESS_MAX: u16 = 299;
const STATUS_SUCCESS_MIN: u16 = 200;
const STATUS_TOO_MANY_REQUESTS: u16 = 429;
const WAIT_REASON_LIMITED: &str = "rate limited";
const WAIT_REASON_PACED: &str = "near the limit";
const WAIT_REASON_SERVER: &str = "server error";
const WAIT_REASON_TRANSPORT: &str = "connection lost";

const _: () =
    assert!(RETRY_AFTER_SECONDS_MAX >= RETRY_DELAY_SECONDS_MAX, "Confluence may ask for longer");

const _: () = assert!(RETRY_COUNT_MAX >= 1, "a failure is retried at least once");
const _: () = assert!(RETRY_DELAY_SECONDS_BASE >= 1, "a retry waits at least a second");

const _: () = assert!(
    RETRY_DELAY_SECONDS_BASE <= RETRY_DELAY_SECONDS_MAX,
    "the first delay sits under the cap",
);

const _: () =
    assert!(RETRY_JITTER_PERCENT_MIN <= RETRY_JITTER_PERCENT_MAX, "the jitter band is ordered");

const _: () = assert!(STATUS_MIN < STATUS_SERVER_ERROR_MIN, "a server error is a status");
const _: () = assert!(STATUS_SERVER_ERROR_MIN <= STATUS_MAX, "a server error is a status");

const _: () =
    assert!(PACE_MILLIS_NEAR_LIMIT <= limits::WAIT_MILLIS_MAX, "a pause fits the wait cap");

const _: () = assert!(
    RETRY_AFTER_SECONDS_MAX * RETRY_JITTER_PERCENT_MAX * 10 == limits::WAIT_MILLIS_MAX,
    "the longest wait is the longest delay Confluence may ask for, at the top of the jitter band",
);

pub struct Client<Network> {
    authorization: String,
    cloud_id: String,
    network: Network,
    prng: Mutex<Prng>,
}

#[derive(Clone, Copy, Debug)]
struct Jitter {
    delay_seconds: u64,
    entropy: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Repeat {
    Any,
    Throttled,
}

impl<Network> Client<Network>
where
    Network: Transport,
{
    pub fn download<Called, Sink>(&self, operation: &Called, sink: &mut Sink) -> Result<u64, Error>
    where
        Called: Operation<Reply = Binary>,
        Sink: Seek + Write,
    {
        self.download_with(operation, &Called::Options::default(), sink)
    }

    pub fn download_with<Called, Sink>(
        &self,
        operation: &Called,
        options: &Called::Options<'_>,
        sink: &mut Sink,
    ) -> Result<u64, Error>
    where
        Called: Operation<Reply = Binary>,
        Sink: Seek + Write,
    {
        let mut request = self.request_build(operation, options)?;

        request.accept_any();

        assert_eq!(request.accept(), "*/*", "a download takes whatever Confluence sends");

        let start = sink
            .stream_position()
            .map_err(|cause| Error::Input(format!("cannot read the download position: {cause}")))?;

        let response = self.retried(&request, || {
            let _position = sink
                .seek(SeekFrom::Start(start))
                .map_err(|cause| Error::Input(format!("cannot rewind the download: {cause}")))?;

            self.network.stream(&request, sink)
        })?;

        status_check(&response, &request)?;

        if response.streamed_count > limits::ATTACHMENT_BYTES_MAX {
            return Err(Error::Reply(format!(
                "a download ran past the {} byte guard",
                limits::ATTACHMENT_BYTES_MAX,
            )));
        }

        assert!(response.body.is_empty(), "a download lands in its sink, not in memory");

        Ok(response.streamed_count)
    }

    fn exchange(&self, request: &Request) -> Result<Response, Error> {
        assert!(!request.url().is_empty(), "a request names where it goes");

        let response =
            self.retried(request, || self.network.fetch(request, limits::BODY_BYTES_MAX))?;

        status_check(&response, request)?;

        assert!(size::of_slice(&response.body) <= limits::BODY_BYTES_MAX, "a body fits its limit");

        Ok(response)
    }

    #[must_use]
    pub fn network(&self) -> &Network {
        assert!(!self.authorization.is_empty(), "a client carries its credential");
        assert!(!self.cloud_id.is_empty(), "a client names its site");

        &self.network
    }

    pub fn new(network: Network, credentials: &Credentials<'_>) -> Result<Self, Error> {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(1, |since| u64::from(since.subsec_nanos()) ^ since.as_secs());

        let client = Self {
            authorization: credentials.authorization()?,
            cloud_id: credentials.cloud_id()?.to_owned(),
            network,
            prng: Mutex::new(Prng::from_seed(seed)),
        };

        assert!(!client.authorization.is_empty(), "a client carries its credential");
        assert!(!client.cloud_id.is_empty(), "a client names its site");

        Ok(client)
    }

    pub(crate) fn page_fetch(&self, request: &Request) -> Result<Value, Error> {
        assert!(request.is_read(), "a walk only ever reads");

        let response = self.exchange(request)?;

        assert!(status_is_success(response.status, request), "only a success reads as a page");

        serde_json::from_slice(&response.body)
            .map_err(|cause| Error::Reply(format!("invalid JSON from {}: {cause}", request.url())))
    }

    pub(crate) fn page_request<Walked>(
        &self,
        operation: &Walked,
        options: &Walked::Options<'_>,
        cursor: &Cursor,
    ) -> Result<Request, Error>
    where
        Walked: Paged,
    {
        let mut request = self.request_build(operation, options)?;

        if let Some(text) = cursor.text() {
            request.query_replace(Walked::PAGING.cursor, text);
        }

        request.url_check()?;

        assert!(request.is_read(), "a walk only ever reads");

        Ok(request)
    }

    fn request_build<Called>(
        &self,
        operation: &Called,
        options: &Called::Options<'_>,
    ) -> Result<Request, Error>
    where
        Called: Operation,
    {
        assert!(!Called::PATH.is_empty(), "an operation names its path");
        assert!(Called::PATH.starts_with('/'), "a path is rooted at its base");

        let url = format!("{}{}", Called::BASE.url(&self.cloud_id), Called::PATH);
        let mut request = Request::of::<Called>(url, &self.authorization);

        operation.request(options, &mut request)?;

        request.content_type_settle();
        request.url_check()?;

        assert_eq!(request.method(), Called::METHOD, "a request keeps its operation's method");

        Ok(request)
    }

    fn retried<Attempt>(
        &self,
        request: &Request,
        mut attempt_run: Attempt,
    ) -> Result<Response, Error>
    where
        Attempt: FnMut() -> Result<Response, Error>,
    {
        assert!(request.url().starts_with("https://"), "every request is encrypted");

        let repeat = if request.is_read() { Repeat::Any } else { Repeat::Throttled };
        let mut delay_seconds = RETRY_DELAY_SECONDS_BASE;
        let mut attempt = attempt_run();

        for retry_index in 0..RETRY_COUNT_MAX {
            let Some(seconds) = retry_delay_seconds(&attempt, repeat, delay_seconds) else {
                break;
            };

            let entropy = self.prng.lock().map_or(0, |mut prng| prng.int());
            let jittered = retry_delay_jitter(Jitter { delay_seconds: seconds, entropy });
            let delay = jittered.max(retry_after_floor(&attempt));

            let note =
                format!("{} for {}", retry_note(&attempt, retry_index, delay), request.url());

            self.network.wait(&Wait {
                delay,
                note: Some(&note),
                reason: retry_reason(&attempt),
            })?;

            delay_seconds = delay_seconds.strict_mul(2).min(RETRY_DELAY_SECONDS_MAX);
            attempt = attempt_run();
        }

        let response = attempt.map_err(|failure| failure_final(failure, request))?;

        assert!((STATUS_MIN..=STATUS_MAX).contains(&response.status), "an answer has a status");

        if response.is_near_limit {
            let pause = Wait {
                delay: Duration::from_millis(PACE_MILLIS_NEAR_LIMIT),
                note: None,
                reason: WAIT_REASON_PACED,
            };

            if request.is_read() {
                self.network.wait(&pause)?;
            } else {
                drop(self.network.wait(&pause));
            }
        }

        Ok(response)
    }

    pub fn send<Called>(&self, operation: &Called) -> Result<Reply<Called::Reply>, Error>
    where
        Called: Operation,
        Called::Reply: Kind,
    {
        self.send_with(operation, &Called::Options::default())
    }

    pub fn send_with<Called>(
        &self,
        operation: &Called,
        options: &Called::Options<'_>,
    ) -> Result<Reply<Called::Reply>, Error>
    where
        Called: Operation,
        Called::Reply: Kind,
    {
        let request = self.request_build(operation, options)?;
        let response = self.exchange(&request)?;

        assert!(status_is_success(response.status, &request), "only a success reads as a reply");

        Reply::parse(response, &request)
    }

    pub fn walk<'call, Walked>(&self, operation: &'call Walked) -> Walk<'_, 'call, Network, Walked>
    where
        Walked: Paged,
    {
        let options = Walked::Options::default();

        self.walk_with(operation, options)
    }

    pub fn walk_with<'call, Walked>(
        &self,
        operation: &'call Walked,
        options: Walked::Options<'call>,
    ) -> Walk<'_, 'call, Network, Walked>
    where
        Walked: Paged,
    {
        Walk::new(self, operation, options)
    }
}

impl<Network> fmt::Debug for Client<Network> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("Client").field("cloud_id", &self.cloud_id).finish_non_exhaustive()
    }
}

fn failure_final(failure: Error, request: &Request) -> Error {
    assert!(!request.url().is_empty(), "a failure names where it happened");

    let Error::Transport(cause) = failure else {
        return failure;
    };

    if request.is_read() {
        return Error::Transport(format!("request failed for {}: {cause}", request.url()));
    }

    Error::Indefinite(format!(
        "{} {} may have landed: {cause}",
        request.method().text(),
        request.url(),
    ))
}

fn retry_after_floor(attempt: &Result<Response, Error>) -> Duration {
    let seconds = attempt.as_ref().ok().and_then(|response| response.retry_after_seconds);
    let floor = seconds.filter(|found| *found <= RETRY_AFTER_SECONDS_MAX).unwrap_or_default();

    assert!(floor <= RETRY_AFTER_SECONDS_MAX, "a floor never asks past the guard");

    Duration::from_secs(floor)
}

fn retry_delay_jitter(jitter: Jitter) -> Duration {
    assert!(jitter.delay_seconds >= 1, "a retry waits at least a second");
    assert!(jitter.delay_seconds <= RETRY_AFTER_SECONDS_MAX, "a retry waits at most the cap");

    let percent_span = RETRY_JITTER_PERCENT_MAX.strict_sub(RETRY_JITTER_PERCENT_MIN).strict_add(1);
    let percent = RETRY_JITTER_PERCENT_MIN.strict_add(jitter.entropy.rem_euclid(percent_span));

    assert!(percent >= RETRY_JITTER_PERCENT_MIN, "the jitter stays inside its band");
    assert!(percent <= RETRY_JITTER_PERCENT_MAX, "the jitter stays inside its band");

    let millis = jitter.delay_seconds.strict_mul(1_000).strict_mul(percent).div_euclid(100);

    Duration::from_millis(millis)
}

fn retry_delay_seconds(
    attempt: &Result<Response, Error>,
    repeat: Repeat,
    delay_seconds: u64,
) -> Option<u64> {
    assert!(delay_seconds >= 1, "a retry waits at least a second");
    assert!(delay_seconds <= RETRY_DELAY_SECONDS_MAX, "a retry waits at most the cap");

    let response = match attempt {
        Ok(response) => response,
        Err(Error::Transport(_)) if repeat == Repeat::Any => return Some(delay_seconds),
        Err(_) => return None,
    };

    if !status_is_retryable(response.status, repeat) {
        return None;
    }

    let Some(seconds) = response.retry_after_seconds else {
        return Some(delay_seconds);
    };

    if seconds > RETRY_AFTER_SECONDS_MAX {
        return None;
    }

    Some(seconds.max(1))
}

fn retry_note(attempt: &Result<Response, Error>, retry_index: u32, delay: Duration) -> String {
    assert!(retry_index < RETRY_COUNT_MAX, "a retry note numbers a real retry");
    assert!(delay.as_millis() <= u128::from(limits::WAIT_MILLIS_MAX), "a retry fits its cap");

    let cause = match attempt {
        Ok(response) => response.rate_limit_reason.as_ref().map_or_else(
            || format!("status {}", response.status),
            |reason| format!("status {} ({reason})", response.status),
        ),
        Err(failure) => format!("transport failure ({failure})"),
    };

    let note = format!(
        "{cause}; retry {} of {RETRY_COUNT_MAX} in {}ms",
        retry_index.strict_add(1),
        delay.as_millis(),
    );

    assert!(note.contains("retry "), "a note names the retry");
    assert!(note.ends_with("ms"), "a note names the delay");

    note
}

fn retry_reason(attempt: &Result<Response, Error>) -> &'static str {
    let Ok(response) = attempt else {
        return WAIT_REASON_TRANSPORT;
    };

    assert!(
        (STATUS_MIN..=STATUS_MAX).contains(&response.status),
        "a retried response carries a real status",
    );

    if response.status == STATUS_TOO_MANY_REQUESTS {
        return WAIT_REASON_LIMITED;
    }

    WAIT_REASON_SERVER
}

fn status_check(response: &Response, request: &Request) -> Result<(), Error> {
    assert!((STATUS_MIN..=STATUS_MAX).contains(&response.status), "a status is real");

    let status = response.status;
    let url = request.url();

    if status_is_success(status, request) {
        return Ok(());
    }

    let excerpt_bytes = size::of_slice(&response.body).min(limits::ERROR_EXCERPT_BYTES_MAX);
    let excerpt_raw = response.body.get(..size::offset(excerpt_bytes)).unwrap_or_default();
    let excerpt = String::from_utf8_lossy(excerpt_raw);
    let message = format!("request failed with {status} for {url}: {excerpt}");

    if status_may_have_landed(status, request) {
        return Err(Error::Indefinite(message));
    }

    Err(Error::Confluence { message, status })
}

fn status_is_retryable(status: u16, repeat: Repeat) -> bool {
    assert!((STATUS_MIN..=STATUS_MAX).contains(&status), "only a real status is judged");

    if status == STATUS_TOO_MANY_REQUESTS {
        return true;
    }

    repeat == Repeat::Any && status >= STATUS_SERVER_ERROR_MIN
}

fn status_is_success(status: u16, request: &Request) -> bool {
    assert!((STATUS_MIN..=STATUS_MAX).contains(&status), "only a real status is judged");

    if (STATUS_SUCCESS_MIN..=STATUS_SUCCESS_MAX).contains(&status) {
        return true;
    }

    if request.is_read() {
        return false;
    }

    status == STATUS_SEE_OTHER
}

fn status_may_have_landed(status: u16, request: &Request) -> bool {
    assert!((STATUS_MIN..=STATUS_MAX).contains(&status), "only a real status is judged");

    if request.is_read() {
        return false;
    }

    status >= STATUS_SERVER_ERROR_MIN
}

#[cfg(test)]
mod tests {
    use core::time::Duration;

    use super::{
        Jitter,
        RETRY_AFTER_SECONDS_MAX,
        RETRY_COUNT_MAX,
        Repeat,
        Response,
        WAIT_REASON_LIMITED,
        WAIT_REASON_SERVER,
        WAIT_REASON_TRANSPORT,
        retry_delay_jitter,
        retry_delay_seconds,
        retry_note,
        retry_reason,
        status_check,
        status_is_retryable,
        status_is_success,
    };
    use crate::base::Base;
    use crate::error::Error;
    use crate::method::Method;
    use crate::operation::Operation;
    use crate::prng::Prng;
    use crate::request::Request;

    #[derive(Debug)]
    struct Read;

    #[derive(Debug)]
    struct Write;

    impl Operation for Read {
        const BASE: Base = Base::Confluence;
        const IS_READ: bool = true;
        const METHOD: Method = Method::Get;
        const PATH: &'static str = "/read";

        type Options<'options> = ();
        type Reply = ();

        fn request(&self, _options: &(), request: &mut Request) -> Result<(), Error> {
            assert!(request.is_blank(), "a request starts blank");
            assert!(request.is_complete(), "a fixture has no placeholder");

            Ok(())
        }
    }

    impl Operation for Write {
        const BASE: Base = Base::Confluence;
        const IS_READ: bool = false;
        const METHOD: Method = Method::Post;
        const PATH: &'static str = "/write";

        type Options<'options> = ();
        type Reply = ();

        fn request(&self, _options: &(), request: &mut Request) -> Result<(), Error> {
            assert!(request.is_blank(), "a request starts blank");
            assert!(request.is_complete(), "a fixture has no placeholder");

            Ok(())
        }
    }

    #[test]
    fn a_read_retries_what_a_write_must_not() {
        let server = Ok(response_build(503, None));
        let reset = Err(failure_build("reset"));
        let throttled = Ok(response_build(429, Some(7)));

        assert_eq!(retry_delay_seconds(&server, Repeat::Any, 5), Some(5), "a read repeats");
        assert_eq!(retry_delay_seconds(&reset, Repeat::Any, 5), Some(5), "a read repeats");
        assert_eq!(retry_delay_seconds(&server, Repeat::Throttled, 5), None, "a write may be in");
        assert_eq!(retry_delay_seconds(&reset, Repeat::Throttled, 5), None, "a write may be in");
        assert_eq!(retry_delay_seconds(&throttled, Repeat::Throttled, 5), Some(7), "never landed");
    }

    #[test]
    fn a_redirect_on_a_write_is_a_task_accepted_and_on_a_read_a_failure_to_follow() {
        assert!(status_is_success(303, &request_of::<Write>()), "a write was accepted");
        assert!(!status_is_success(303, &request_of::<Read>()), "a read should have followed");
        assert!(status_is_success(204, &request_of::<Write>()), "a 204 is a success");
        assert!(!status_is_success(404, &request_of::<Write>()), "a 404 is a refusal");
    }

    #[test]
    fn a_retry_after_beyond_the_cap_ends_the_retries() {
        let past = RETRY_AFTER_SECONDS_MAX.strict_add(1);

        for (seconds, expected) in [(past, None), (86_400, None), (31, Some(31)), (300, Some(300))]
        {
            assert_eq!(
                retry_delay_seconds(&Ok(response_build(429, Some(seconds))), Repeat::Any, 5),
                expected,
                "{seconds} seconds",
            );
        }

        assert_eq!(
            retry_delay_seconds(&Ok(response_build(200, None)), Repeat::Any, 5),
            None,
            "a success",
        );

        assert_eq!(
            retry_delay_seconds(&Ok(response_build(404, None)), Repeat::Any, 5),
            None,
            "a refusal",
        );
    }

    #[test]
    fn a_retry_note_names_the_cause_the_attempt_and_the_delay() {
        assert_eq!(
            retry_note(&Ok(throttled_build("EXCEEDED_COST")), 0, Duration::from_millis(4_200)),
            "status 429 (EXCEEDED_COST); retry 1 of 4 in 4200ms",
            "a throttle names its reason",
        );

        assert_eq!(
            retry_note(&Ok(response_build(503, None)), 3, Duration::from_secs(30)),
            "status 503; retry 4 of 4 in 30000ms",
            "a server error names its status",
        );

        let transport =
            retry_note(&Err(failure_build("connection reset")), 1, Duration::from_secs(5));

        assert!(transport.starts_with("transport failure (connection reset)"), "the cause");
        assert!(transport.contains("retry 2 of 4"), "the attempt");
    }

    #[test]
    #[should_panic(expected = "a retry note numbers a real retry")]
    fn a_retry_note_past_the_retry_ceiling_is_a_programmer_error() {
        let _note =
            retry_note(&Ok(response_build(429, None)), RETRY_COUNT_MAX, Duration::from_millis(1));
    }

    #[test]
    fn a_server_failure_on_a_write_may_have_landed_and_on_a_read_it_has_not() {
        let write = status_check(&response_build(504, None), &request_of::<Write>());
        let read = status_check(&response_build(504, None), &request_of::<Read>());
        let refused = status_check(&response_build(400, None), &request_of::<Write>());

        let read_status = read.err().and_then(|found| found.status());
        let refused_status = refused.err().and_then(|found| found.status());

        assert!(matches!(write, Err(Error::Indefinite(_))), "the gateway may have let it through");
        assert_eq!(read_status, Some(504), "a read changed nothing");
        assert_eq!(refused_status, Some(400), "a 4xx never landed");
    }

    #[test]
    fn a_status_outside_success_is_a_refusal_with_an_excerpt_of_the_body() {
        let mut response = response_build(404, None);

        response.body = b"{\"errorMessages\":[\"gone\"]}".to_vec();

        let request = request_of::<Read>();
        let refused = status_check(&response, &request).expect_err("a 404 is refused");

        assert_eq!(refused.status(), Some(404), "the status is kept");
        assert!(refused.message().contains("errorMessages"), "the body is excerpted");
        assert!(status_check(&response_build(204, None), &request).is_ok(), "a 204 passes");
    }

    #[test]
    fn a_wait_names_throttling_apart_from_a_failure_it_cannot_see() {
        assert_eq!(retry_reason(&Ok(response_build(429, None))), WAIT_REASON_LIMITED, "a throttle");
        assert_eq!(retry_reason(&Ok(response_build(503, None))), WAIT_REASON_SERVER, "a server");

        assert_eq!(
            retry_reason(&Err(failure_build("reset"))),
            WAIT_REASON_TRANSPORT,
            "the network",
        );
    }

    fn failure_build(cause: &str) -> Error {
        assert!(!cause.is_empty(), "a fixture failure says why");

        let failure = Error::Transport(cause.to_owned());

        assert_eq!(failure.message(), cause, "a fixture failure keeps its cause");

        failure
    }

    fn request_of<Called>() -> Request
    where
        Called: Operation,
    {
        let url = format!("https://api.atlassian.com/ex/confluence/site{}", Called::PATH);
        let request = Request::of::<Called>(url, "Basic secret");

        assert!(request.is_blank(), "a fixture request starts blank");
        assert_eq!(request.method(), Called::METHOD, "a fixture request keeps its method");

        request
    }

    fn response_build(status: u16, retry_after_seconds: Option<u64>) -> Response {
        assert!((100..=599).contains(&status), "a fixture response carries a real status");

        assert!(
            retry_after_seconds.is_none_or(|seconds| seconds <= 86_400),
            "a fixture waits at most a day",
        );

        Response {
            body: Vec::new(),
            is_near_limit: false,
            location: None,
            rate_limit_reason: None,
            retry_after_seconds,
            status,
            streamed_count: 0,
        }
    }

    #[test]
    fn retry_delay_honors_the_server_within_the_cap() {
        for (seconds, expected) in [(10, Some(10)), (0, Some(1))] {
            assert_eq!(
                retry_delay_seconds(&Ok(response_build(429, Some(seconds))), Repeat::Any, 5),
                expected,
                "{seconds} seconds",
            );
        }

        assert_eq!(
            retry_delay_seconds(&Ok(response_build(429, None)), Repeat::Any, 5),
            Some(5),
            "the backoff",
        );

        assert_eq!(
            retry_delay_seconds(&Ok(response_build(503, None)), Repeat::Any, 20),
            Some(20),
            "the backoff",
        );
    }

    #[test]
    #[should_panic(expected = "a retry waits at most the cap")]
    fn retry_delay_jitter_refuses_a_delay_over_the_cap() {
        let _delay = retry_delay_jitter(Jitter { delay_seconds: 901, entropy: 0 });
    }

    #[test]
    fn retry_delay_jitter_stays_inside_the_declared_band() {
        let mut prng = Prng::from_seed(123);

        for _sample_index in 0..4_096_u32 {
            let delay = retry_delay_jitter(Jitter { delay_seconds: 5, entropy: prng.int() });

            assert!(delay.as_millis() >= 3_500, "the jitter floor holds");
            assert!(delay.as_millis() <= 6_500, "the jitter ceiling holds");
        }
    }

    #[test]
    fn status_is_retryable_covers_throttling_and_gateway_failures() {
        for status in [429, 500, 502, 503, 599] {
            assert!(status_is_retryable(status, Repeat::Any), "{status} is retried");
        }

        for status in [200, 400, 401, 404, 499] {
            assert!(!status_is_retryable(status, Repeat::Any), "{status} is final");
        }

        assert!(status_is_retryable(429, Repeat::Throttled), "a throttled write never landed");
        assert!(!status_is_retryable(503, Repeat::Throttled), "a failed write may have landed");
    }

    #[test]
    #[should_panic(expected = "only a real status is judged")]
    fn status_is_retryable_refuses_an_impossible_status() {
        let _retryable = status_is_retryable(600, Repeat::Any);
    }

    #[test]
    fn the_jitter_band_reaches_both_of_its_declared_ends() {
        for (entropy, seconds) in [(0, 7), (60, 13), (61, 7)] {
            assert_eq!(
                retry_delay_jitter(Jitter { delay_seconds: 10, entropy }),
                Duration::from_secs(seconds),
                "entropy {entropy} lands at {seconds} seconds",
            );
        }
    }

    fn throttled_build(reason: &str) -> Response {
        assert!(!reason.is_empty(), "a throttle names its reason");

        Response {
            body: Vec::new(),
            is_near_limit: false,
            location: None,
            rate_limit_reason: Some(reason.to_owned()),
            retry_after_seconds: Some(5),
            status: 429,
            streamed_count: 0,
        }
    }
}
