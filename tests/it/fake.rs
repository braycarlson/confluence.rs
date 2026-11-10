use core::sync::atomic::{AtomicBool, Ordering};
use std::io::{Seek, Write};
use std::sync::Mutex;

use confluence::{Client, Credentials, Error, Method, Request, Response, Transport, Wait};
use serde_json::Value;

pub(crate) const CLOUD_ID: &str = "11111111-2222-3333-4444-555555555555";

#[derive(Debug)]
pub(crate) struct FakeNetwork {
    answers: Mutex<Vec<Scripted>>,
    is_wait_refused: AtomicBool,
    seen: Mutex<Vec<Seen>>,
    waits: Mutex<Vec<Waited>>,
}

#[derive(Debug)]
pub(crate) enum Scripted {
    Answered(Response),
    Dropped,
}

#[derive(Clone, Debug)]
pub(crate) struct Seen {
    pub accept: String,
    pub authorization: String,
    pub body: Vec<u8>,
    pub content_type: Option<String>,
    pub headers: Vec<(String, String)>,
    pub is_read: bool,
    pub method: Method,
    pub url: String,
}

#[derive(Clone, Debug)]
pub(crate) struct Waited {
    pub millis: u128,
    pub note: Option<String>,
    pub reason: &'static str,
}

impl FakeNetwork {
    fn answer_next(&self, request: &Request) -> Result<Response, Error> {
        assert!(request.url().starts_with("https://"), "every request is encrypted");
        assert!(!request.authorization().is_empty(), "every request carries a credential");

        self.seen_push(request);

        match self.scripted_next() {
            Scripted::Answered(response) => Ok(response),
            Scripted::Dropped => Err(Error::Transport("connection reset".to_owned())),
        }
    }

    pub(crate) fn answering(answers: Vec<Scripted>) -> Self {
        assert!(!answers.is_empty(), "a script answers at least one request");
        assert!(answers.len() <= 64, "a script is short");

        let network = Self {
            answers: Mutex::new(answers),
            is_wait_refused: AtomicBool::new(false),
            seen: Mutex::new(Vec::new()),
            waits: Mutex::new(Vec::new()),
        };

        assert!(network.answers_left() >= 1, "a fresh script holds its answers");
        assert!(network.seen().is_empty(), "a fresh network has seen nothing");

        network
    }

    pub(crate) fn answers_left(&self) -> usize {
        let count = self.answers.lock().expect("the script").len();

        assert!(count <= 64, "a script is short");

        count
    }

    fn scripted_next(&self) -> Scripted {
        let mut answers = self.answers.lock().expect("the script");

        assert!(!answers.is_empty(), "a request arrived that the script did not expect");
        assert!(answers.len() <= 64, "a script is short");

        answers.remove(0)
    }

    pub(crate) fn seen(&self) -> Vec<Seen> {
        let seen = self.seen.lock().expect("the request log").clone();

        assert!(seen.len() <= 64, "a test makes few requests");
        assert!(seen.iter().all(|found| found.url.starts_with("https://")), "each is encrypted");

        seen
    }

    fn seen_push(&self, request: &Request) {
        assert!(request.url().starts_with("https://"), "every request is encrypted");
        assert!(!request.accept().is_empty(), "every request names what it accepts");

        self.seen.lock().expect("the request log").push(Seen {
            accept: request.accept().to_owned(),
            authorization: request.authorization().to_owned(),
            body: request.body().to_vec(),
            content_type: request.content_type().map(str::to_owned),
            headers: request
                .headers()
                .iter()
                .map(|(name, value)| ((*name).to_owned(), value.clone()))
                .collect(),
            is_read: request.is_read(),
            method: request.method(),
            url: request.url().to_owned(),
        });
    }

    fn sink_fill<Sink>(sink: &mut Sink, response: &Response)
    where
        Sink: Seek + Write,
    {
        assert!(response.status >= 100, "an answer carries a real status");
        assert!(response.status <= 599, "an answer carries a real status");

        sink.write_all(&response.body).expect("write the fixture body");
    }

    fn wait_push(&self, wait: &Wait<'_>) {
        assert!(!wait.reason.is_empty(), "a wait says why");
        assert!(!wait.delay.is_zero(), "a wait waits");

        self.waits.lock().expect("the wait log").push(Waited {
            millis: wait.delay.as_millis(),
            note: wait.note.map(str::to_owned),
            reason: wait.reason,
        });
    }

    pub(crate) fn wait_refuse(&self) {
        self.is_wait_refused.store(true, Ordering::Relaxed);

        assert!(self.is_wait_refused.load(Ordering::Relaxed), "the next wait fails");
        assert!(self.answers_left() <= 64, "a script is short");
    }

    pub(crate) fn waits(&self) -> Vec<Waited> {
        let waits = self.waits.lock().expect("the wait log").clone();

        assert!(waits.len() <= 64, "a test waits few times");

        waits
    }
}

impl Transport for FakeNetwork {
    fn fetch(&self, request: &Request, bytes_max: u64) -> Result<Response, Error> {
        assert!(bytes_max >= 1, "a body may hold at least one byte");
        assert!(request.url().starts_with("https://"), "every request is encrypted");

        self.answer_next(request)
    }

    fn stream<Sink>(&self, request: &Request, sink: &mut Sink) -> Result<Response, Error>
    where
        Sink: Seek + Write,
    {
        assert!(request.url().starts_with("https://"), "every request is encrypted");

        let mut response = self.answer_next(request)?;

        Self::sink_fill(sink, &response);

        let body_count = u64::try_from(response.body.len()).unwrap_or(u64::MAX);

        response.streamed_count = response.streamed_count.max(body_count);
        response.body = Vec::new();

        assert!(response.body.is_empty(), "a stream lands in its sink");

        Ok(response)
    }

    fn wait(&self, wait: &Wait<'_>) -> Result<(), Error> {
        assert!(!wait.reason.is_empty(), "a wait says why");
        assert!(!wait.delay.is_zero(), "a wait waits");

        self.wait_push(wait);

        if self.is_wait_refused.load(Ordering::Relaxed) {
            return Err(Error::Input("the console went away".to_owned()));
        }

        Ok(())
    }
}

pub(crate) fn answer(status: u16, body: &Value) -> Scripted {
    assert!((100..=599).contains(&status), "a fixture answer carries a real status");

    let scripted = Scripted::Answered(response(status, body));

    assert!(matches!(scripted, Scripted::Answered(_)), "a fixture answer is an answer");

    scripted
}

pub(crate) fn client(answers: Vec<Scripted>) -> Client<FakeNetwork> {
    assert!(!answers.is_empty(), "a script answers at least one request");

    let credentials =
        Credentials::Token { cloud_id: CLOUD_ID, email: "user@example.com", token: "token123" };

    let network = FakeNetwork::answering(answers);

    assert!(network.waits().is_empty(), "a fresh network has waited for nothing");

    Client::new(network, &credentials).expect("the fixture credentials")
}

pub(crate) fn dropped() -> Scripted {
    let scripted = Scripted::Dropped;

    assert!(matches!(scripted, Scripted::Dropped), "a fixture failure is a failure");

    scripted
}

pub(crate) fn response(status: u16, body: &Value) -> Response {
    assert!((100..=599).contains(&status), "a fixture answer carries a real status");

    let bytes =
        if body.is_null() { Vec::new() } else { serde_json::to_vec(body).expect("a fixture body") };

    Response {
        body: bytes,
        is_near_limit: false,
        location: None,
        rate_limit_reason: None,
        retry_after_seconds: None,
        status,
        streamed_count: 0,
    }
}

pub(crate) fn url_of(path: &str) -> String {
    assert!(path.starts_with('/'), "a path is rooted at its base");
    assert!(!path.contains(' '), "a fixture path is already escaped");

    let url = format!("https://api.atlassian.com/ex/confluence/{CLOUD_ID}{path}");

    assert!(url.contains(CLOUD_ID), "a fixture URL names the fixture site");
    assert!(url.starts_with("https://"), "every fixture URL is encrypted");

    url
}
