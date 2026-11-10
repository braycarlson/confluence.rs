use core::time::Duration;
use std::io::{Read as _, Seek, Write};
use std::thread;

use ureq::config::{Config, RedirectAuthHeaders};
#[cfg(test)]
use ureq::unversioned::resolver::Resolver;
#[cfg(test)]
use ureq::unversioned::transport::Connector;
use ureq::{AsSendBody, http};

use crate::error::Error;
use crate::limits;
use crate::method::Method;
use crate::request::Request;
use crate::response::Response;
use crate::size;
use crate::text;
use crate::transport::Transport;
use crate::wait::Wait;

const CONNECTION_IDLE_COUNT_MAX: u64 = 16;
const COPY_BUFFER_BYTES: u64 = 64 * 1024;
const HEADER_BYTES_MAX: u64 = 2_048;
const DOWNLOAD_TIMEOUT_SECONDS: u64 = 3_600;
const REDIRECT_COUNT_MAX: u32 = 8;
const REQUEST_TIMEOUT_SECONDS: u64 = 60;
const STATUS_MAX: u16 = 599;
const STATUS_MIN: u16 = 100;
const STATUS_SUCCESS_MAX: u16 = 299;
const STATUS_SUCCESS_MIN: u16 = 200;
const UPLOAD_BYTES_PER_SECOND_MIN: u64 = 64 * 1024;
const _: () = assert!(CONNECTION_IDLE_COUNT_MAX >= 1, "a client keeps at least one connection");
const _: () = assert!(REQUEST_TIMEOUT_SECONDS >= 1, "a request gets at least a second");

const _: () =
    assert!(DOWNLOAD_TIMEOUT_SECONDS >= REQUEST_TIMEOUT_SECONDS, "a download outlasts a request");

const _: () = assert!(STATUS_MIN < STATUS_MAX, "the status range is ordered");
const _: () = assert!(COPY_BUFFER_BYTES >= 1, "a copy moves at least one byte");
const _: () = assert!(UPLOAD_BYTES_PER_SECOND_MIN >= 1, "an upload is given a floor rate");

#[derive(Clone, Copy, Debug)]
struct Limits {
    redirect_count_max: u32,
    timeout: Duration,
}

#[derive(Debug)]
pub struct Agent {
    ureq: ureq::Agent,
}

impl Agent {
    pub(crate) fn call(
        &self,
        request: &Request,
        timeout: Duration,
    ) -> Result<http::Response<ureq::Body>, Error> {
        assert!(!timeout.is_zero(), "a call is given time to finish");
        assert!(request.url().starts_with("https://"), "every request is encrypted");

        let is_get = request.method() == Method::Get;
        let redirect_count_max = if is_get { REDIRECT_COUNT_MAX } else { 0 };

        let answer = if request.body().is_empty() {
            self.run(self.outbound(request, ())?, Limits { redirect_count_max, timeout })
        } else {
            self.run(
                self.outbound(request, request.body())?,
                Limits { redirect_count_max, timeout },
            )
        };

        let response = answer.map_err(|cause| Error::Transport(cause.to_string()))?;
        let status = response.status().as_u16();

        if !(STATUS_MIN..=STATUS_MAX).contains(&status) {
            return Err(Error::Transport(format!("an impossible status {status}")));
        }

        assert!(status >= STATUS_MIN, "an answer carries a real status");
        assert!(status <= STATUS_MAX, "an answer carries a real status");

        Ok(response)
    }

    fn config() -> Config {
        let config = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .https_only(true)
            .redirect_auth_headers(RedirectAuthHeaders::Never)
            .max_idle_connections(size::offset(CONNECTION_IDLE_COUNT_MAX))
            .max_idle_connections_per_host(size::offset(CONNECTION_IDLE_COUNT_MAX))
            .build();

        assert!(config.https_only(), "a request never leaves TLS");

        config
    }

    #[must_use]
    pub fn new() -> Self {
        let agent = Self { ureq: Self::config().new_agent() };

        assert!(!agent.ureq.config().http_status_as_error(), "a refusal is read, not raised");
        assert!(agent.ureq.config().max_idle_connections() >= 1, "a client keeps a connection");

        agent
    }

    fn outbound<Payload>(
        &self,
        request: &Request,
        body: Payload,
    ) -> Result<http::Request<Payload>, Error> {
        assert!(!request.url().is_empty(), "a request names where it goes");
        assert!(self.ureq.config().max_idle_connections() >= 1, "an agent keeps a connection");

        let mut builder = http::Request::builder()
            .method(request.method().text())
            .uri(request.url())
            .header("Accept", request.accept())
            .header("Authorization", request.authorization());

        if let Some(content_type) = request.content_type() {
            builder = builder.header("Content-Type", content_type);
        }

        for (name, value) in request.headers() {
            builder = builder.header(*name, value.as_str());
        }

        builder.body(body).map_err(|cause| Error::Input(format!("a bad request: {cause}")))
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn over<Over, Resolving>(connector: Over, resolver: Resolving) -> Self
    where
        Over: Connector,
        Resolving: Resolver,
    {
        let agent = Self { ureq: ureq::Agent::with_parts(Self::config(), connector, resolver) };

        assert!(agent.ureq.config().https_only(), "a test agent keeps the real configuration");

        agent
    }

    fn run<Payload>(
        &self,
        outbound: http::Request<Payload>,
        limits: Limits,
    ) -> Result<http::Response<ureq::Body>, ureq::Error>
    where
        Payload: AsSendBody,
    {
        assert!(limits.redirect_count_max <= REDIRECT_COUNT_MAX, "a call follows few redirects");
        assert!(!limits.timeout.is_zero(), "a call is given time to finish");

        let configured = self
            .ureq
            .configure_request(outbound)
            .max_redirects(limits.redirect_count_max)
            .timeout_global(Some(limits.timeout))
            .build();

        self.ureq.run(configured)
    }
}

impl Default for Agent {
    fn default() -> Self {
        Self::new()
    }
}

impl Transport for Agent {
    fn fetch(&self, request: &Request, bytes_max: u64) -> Result<Response, Error> {
        assert!(bytes_max >= 1, "a body may hold at least one byte");
        assert!(bytes_max <= limits::BODY_BYTES_MAX, "a body fits the largest page");

        let mut answer = self.call(request, timeout_of(request))?;

        let body = answer
            .body_mut()
            .with_config()
            .limit(bytes_max)
            .read_to_vec()
            .map_err(|cause| Error::Transport(format!("body read failed: {cause}")))?;

        assert!(size::of_slice(&body) <= bytes_max, "a body never passes its limit");

        Ok(response_read(&answer, body, 0))
    }

    fn stream<Sink>(&self, request: &Request, sink: &mut Sink) -> Result<Response, Error>
    where
        Sink: Seek + Write,
    {
        assert!(request.url().starts_with("https://"), "every request is encrypted");

        let mut answer = self.call(request, Duration::from_secs(DOWNLOAD_TIMEOUT_SECONDS))?;

        let status = answer.status().as_u16();

        if !(STATUS_SUCCESS_MIN..=STATUS_SUCCESS_MAX).contains(&status) {
            let refusal = refusal_read(&mut answer);

            return Ok(response_read(&answer, refusal, 0));
        }

        let streamed_count = body_copy(&mut answer, sink)?;

        assert!(
            streamed_count <= limits::ATTACHMENT_BYTES_MAX.strict_add(1),
            "a download stops one byte past its guard",
        );

        Ok(response_read(&answer, Vec::new(), streamed_count))
    }

    fn wait(&self, wait: &Wait<'_>) -> Result<(), Error> {
        assert!(!wait.reason.is_empty(), "a wait always says why");
        assert!(self.ureq.config().max_idle_connections() >= 1, "an agent keeps a connection");

        thread::sleep(wait.delay);

        Ok(())
    }
}

fn body_copy<Sink>(answer: &mut http::Response<ureq::Body>, sink: &mut Sink) -> Result<u64, Error>
where
    Sink: Write,
{
    let guard = limits::ATTACHMENT_BYTES_MAX.strict_add(1);
    let mut reader = answer.body_mut().as_reader().take(guard);
    let mut buffer = vec![0_u8; size::offset(COPY_BUFFER_BYTES)];
    let mut copied: u64 = 0;

    assert!(!buffer.is_empty(), "a copy has room for a byte");

    for _read_index in 0..guard {
        let read_count = reader
            .read(&mut buffer)
            .map_err(|cause| Error::Transport(format!("body read failed: {cause}")))?;

        if read_count == 0 {
            break;
        }

        let chunk = buffer.get(..read_count).unwrap_or_default();

        sink.write_all(chunk)
            .map_err(|cause| Error::Input(format!("cannot write the download: {cause}")))?;

        copied = copied.strict_add(size::of_offset(read_count));
    }

    assert!(copied <= guard, "a copy stops at its guard");

    Ok(copied)
}

const fn byte_is_header_name(byte: u8) -> bool {
    if byte.is_ascii_lowercase() {
        return true;
    }

    assert!(!byte.is_ascii_lowercase(), "a lowercase letter already passed");

    byte == b'-'
}

fn header_flag(response: &http::Response<ureq::Body>, name: &str) -> bool {
    assert!(name.bytes().all(byte_is_header_name), "a header name is lowercase");

    response
        .headers()
        .get(name)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.eq_ignore_ascii_case("true"))
}

fn header_number(response: &http::Response<ureq::Body>, name: &str) -> Option<u64> {
    assert!(name.bytes().all(byte_is_header_name), "a header name is lowercase");

    response
        .headers()
        .get(name)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
}

fn header_text(response: &http::Response<ureq::Body>, name: &str) -> Option<String> {
    assert!(name.bytes().all(byte_is_header_name), "a header name is lowercase");

    let value = response.headers().get(name)?.to_str().ok()?;

    if value.is_empty() {
        return None;
    }

    Some(text::prefix(value, HEADER_BYTES_MAX).to_owned())
}

fn header_whole(response: &http::Response<ureq::Body>, name: &str) -> Option<String> {
    assert!(name.bytes().all(byte_is_header_name), "a header name is lowercase");

    let value = response.headers().get(name)?.to_str().ok()?;

    if value.is_empty() {
        return None;
    }

    if size::of_text(value) > u64::from(limits::URL_BYTES_MAX) {
        return None;
    }

    assert!(!value.is_empty(), "a kept header says something");

    Some(value.to_owned())
}

fn refusal_read(answer: &mut http::Response<ureq::Body>) -> Vec<u8> {
    let mut refusal = Vec::new();
    let mut reader = answer.body_mut().as_reader().take(limits::ERROR_EXCERPT_BYTES_MAX);

    if let Err(cause) = reader.read_to_end(&mut refusal) {
        refusal.extend_from_slice(format!(" (the rest was lost: {cause})").as_bytes());
    }

    assert!(refusal.capacity() >= refusal.len(), "a refusal holds what it read");

    refusal
}

fn response_read(
    answer: &http::Response<ureq::Body>,
    body: Vec<u8>,
    streamed_count: u64,
) -> Response {
    let status = answer.status().as_u16();

    assert!((STATUS_MIN..=STATUS_MAX).contains(&status), "a read answer carries a real status");
    assert!(body.is_empty() || streamed_count == 0, "a body is held or streamed, never both");

    Response {
        body,
        is_near_limit: header_flag(answer, "x-ratelimit-nearlimit"),
        location: header_whole(answer, "location"),
        rate_limit_reason: header_text(answer, "ratelimit-reason"),
        retry_after_seconds: header_number(answer, "retry-after"),
        status,
        streamed_count,
    }
}

fn timeout_of(request: &Request) -> Duration {
    let body_bytes = size::of_slice(request.body());
    let body_seconds = body_bytes.div_euclid(UPLOAD_BYTES_PER_SECOND_MIN);
    let seconds = REQUEST_TIMEOUT_SECONDS.strict_add(body_seconds).min(DOWNLOAD_TIMEOUT_SECONDS);

    assert!(seconds >= REQUEST_TIMEOUT_SECONDS, "a large body never shortens the timeout");
    assert!(seconds <= DOWNLOAD_TIMEOUT_SECONDS, "a timeout stays under the longest call");

    Duration::from_secs(seconds)
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use ureq::config::RedirectAuthHeaders;
    use ureq::http;

    use super::{
        Agent,
        REQUEST_TIMEOUT_SECONDS,
        UPLOAD_BYTES_PER_SECOND_MIN,
        header_flag,
        header_number,
        header_text,
        header_whole,
        response_read,
        timeout_of,
    };
    use crate::limits;
    use crate::request::Request;
    use crate::response::Response;
    use crate::size;
    use crate::v2::CreatePage;

    #[test]
    fn a_header_that_cannot_be_read_as_its_kind_reads_as_absent() {
        let answer = answer_of(200, &[("retry-after", "soon"), ("ratelimit-reason", "")]);

        assert_eq!(header_number(&answer, "retry-after"), None, "a date is not a delay");
        assert_eq!(header_number(&answer, "missing"), None, "a missing header");
        assert_eq!(header_text(&answer, "ratelimit-reason"), None, "an empty reason");
        assert!(!header_flag(&answer, "x-ratelimit-nearlimit"), "a missing flag");
    }

    #[test]
    fn a_large_body_is_given_time_in_proportion_to_its_size() {
        let body = "x".repeat(size::offset(UPLOAD_BYTES_PER_SECOND_MIN.strict_mul(3)));
        let url = "https://api.atlassian.com/ex/confluence/site/wiki/api/v2/pages".to_owned();
        let mut request = Request::of::<CreatePage<'_>>(url, "Basic c2VjcmV0");

        assert_eq!(timeout_of(&request).as_secs(), REQUEST_TIMEOUT_SECONDS, "an empty body");

        request.body_json(&Value::String(body)).expect("a body");

        assert_eq!(timeout_of(&request).as_secs(), REQUEST_TIMEOUT_SECONDS.strict_add(3), "scaled");
        assert_eq!(UPLOAD_BYTES_PER_SECOND_MIN, 64 * 1024, "the floor rate is 64 KiB a second");
    }

    #[test]
    fn a_location_at_its_limit_is_kept_and_one_byte_more_is_not() {
        let limit = size::offset(u64::from(limits::URL_BYTES_MAX));
        let fits = "x".repeat(limit);
        let long = "x".repeat(limit.strict_add(1));
        let kept = answer_of(303, &[("location", fits.as_str())]);
        let cut = answer_of(303, &[("location", long.as_str())]);

        assert_eq!(header_whole(&kept, "location").as_deref(), Some(fits.as_str()), "at the limit");
        assert_eq!(header_whole(&cut, "location"), None, "past the limit");
    }

    #[test]
    fn a_location_is_kept_whole_or_not_at_all() {
        let long = format!("https://site/{}", "x".repeat(9_000));
        let answer = answer_of(303, &[("location", long.as_str())]);

        assert_eq!(header_whole(&answer, "location"), None, "a cut address would mislead");
        assert_eq!(header_whole(&answer, "missing"), None, "a missing header");
    }

    #[test]
    fn a_new_agent_reads_a_refusal_rather_than_raising_it() {
        let agent = Agent::new();

        assert!(!agent.ureq.config().http_status_as_error(), "a 404 is an answer");
        assert!(agent.ureq.config().https_only(), "a redirect never leaves TLS");

        assert!(
            matches!(agent.ureq.config().redirect_auth_headers(), RedirectAuthHeaders::Never),
            "a credential never follows a redirect",
        );

        assert_eq!(agent.ureq.config().max_idle_connections(), 16, "the wave keeps its pool");
    }

    #[test]
    fn a_rate_limited_answer_keeps_every_signal_confluence_sends() {
        let answer = answer_of(
            429,
            &[
                ("retry-after", "31"),
                ("ratelimit-reason", "confluence-quota-tenant-based"),
                ("x-ratelimit-nearlimit", "TRUE"),
            ],
        );

        let response = answer_read(&answer);
        let reason = response.rate_limit_reason.as_deref();

        assert_eq!(response.status, 429, "the status");
        assert_eq!(response.retry_after_seconds, Some(31), "the delay Confluence asked for");
        assert_eq!(reason, Some("confluence-quota-tenant-based"), "the reason");
        assert!(response.is_near_limit, "the flag reads in any case");
    }

    #[test]
    fn a_reason_is_cut_to_its_budget() {
        let long = "r".repeat(4_096);
        let answer = answer_of(429, &[("ratelimit-reason", long.as_str())]);
        let response = answer_read(&answer);

        assert_eq!(
            response.rate_limit_reason.map(|reason| reason.len()),
            Some(2_048),
            "the budget",
        );
    }

    #[test]
    fn a_redirect_answer_keeps_where_it_points() {
        let task = "https://site/wiki/rest/api/longtask/10";
        let answer = answer_of(303, &[("location", task)]);
        let response = answer_read(&answer);

        assert_eq!(response.status, 303, "the status");
        assert_eq!(response.location.as_deref(), Some(task), "where it points");
    }

    #[test]
    #[should_panic(expected = "a header name is lowercase")]
    fn an_uppercase_header_name_is_a_programmer_error() {
        let answer = answer_of(200, &[]);
        let flag = header_flag(&answer, "X-RateLimit-NearLimit");

        assert!(!flag, "the panic above is the point of this test");
    }

    fn answer_of(status: u16, headers: &[(&str, &str)]) -> http::Response<ureq::Body> {
        assert!((100..=599).contains(&status), "a fixture answer carries a real status");
        assert!(headers.len() <= 8, "a fixture carries a few headers");

        let mut builder = http::Response::builder().status(status);

        for (name, value) in headers {
            builder = builder.header(*name, *value);
        }

        builder.body(ureq::Body::builder().data("")).expect("a fixture answer")
    }

    fn answer_read(answer: &http::Response<ureq::Body>) -> Response {
        assert!(answer.status().as_u16() >= 100, "a fixture answer carries a real status");
        assert!(answer.status().as_u16() <= 599, "a fixture answer carries a real status");

        response_read(answer, Vec::new(), 0)
    }
}
