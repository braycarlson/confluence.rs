use alloc::sync::Arc;
use core::net::SocketAddr;
use core::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;
use std::io::{self, Cursor, Read as _, Seek, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::thread::{self, JoinHandle};

use serde_json::json;
use ureq::config::Config;
use ureq::http::Uri;
use ureq::unversioned::resolver::{ResolvedSocketAddrs, Resolver};
use ureq::unversioned::transport::{
    Buffers,
    ConnectionDetails,
    Connector,
    NextTimeout,
    TcpConnector,
    Transport as Carrier,
};

use crate::agent::Agent;
use crate::client::Client;
use crate::credentials::Credentials;
use crate::error::Error;
use crate::limits;
use crate::part::Part;
use crate::request::Request;
use crate::response::Response;
use crate::size;
use crate::transport::Transport;
use crate::v1::{CreateAttachment, DeleteLabelFromSpace, DownloadAttatchment, GetCurrentUser};
use crate::v2::{CreatePage, GetDatabaseEntries, GetSpaces, get_database_entries};
use crate::wait::Wait;

const AUTHORIZATION: &str = "Basic c2VjcmV0";
const BYTES_MAX: u64 = 1024 * 1024;
const LINE_WRITE: &str = "POST /ex/confluence/site/wiki/rest/api/user/current HTTP/1.1";
const POLL_COUNT_MAX: u32 = 15_000;
const POLL_MILLIS: u64 = 2;
const READ_COUNT_MAX: u32 = 1_024;
const READ_TIMEOUT_SECONDS: u64 = 30;
const REDIRECT_COUNT_MAX: u32 = 8;
const URL_READ: &str = "https://api.atlassian.com/ex/confluence/site/wiki/rest/api/user/current";

const CREDENTIALS: Credentials<'static> =
    Credentials::Token { cloud_id: "site", email: "user@example.com", token: "token123" };

const DOWNLOAD: DownloadAttatchment<'static> =
    DownloadAttatchment { attachment_id: "att9", id: "1" };

#[derive(Debug)]
struct Disguised {
    carrier: Box<dyn Carrier>,
}

#[derive(Debug)]
struct Instant {
    agent: Agent,
    waits: Mutex<Vec<Duration>>,
}

#[derive(Debug)]
struct Local {
    address: SocketAddr,
}

#[derive(Debug)]
struct Loopback;

#[derive(Clone, Debug)]
enum Scene {
    Hang,
    Say(Vec<u8>),
    Vanish,
}

#[derive(Debug)]
struct Server {
    address: SocketAddr,
    handle: JoinHandle<Vec<String>>,
    is_finished: Arc<AtomicBool>,
}

#[derive(Debug)]
struct SinkRefusing;

impl Carrier for Disguised {
    fn await_input(&mut self, timeout: NextTimeout) -> Result<bool, ureq::Error> {
        self.carrier.await_input(timeout)
    }

    fn buffers(&mut self) -> &mut dyn Buffers {
        self.carrier.buffers()
    }

    fn is_open(&mut self) -> bool {
        self.carrier.is_open()
    }

    fn is_tls(&self) -> bool {
        assert!(format!("{self:?}").contains("carrier"), "a disguise wraps a carrier");

        true
    }

    fn transmit_output(&mut self, amount: usize, timeout: NextTimeout) -> Result<(), ureq::Error> {
        assert!(amount >= 1, "a transmit moves at least one byte");

        self.carrier.transmit_output(amount, timeout)
    }
}

impl Connector for Loopback {
    type Out = Disguised;

    fn connect(
        &self,
        details: &ConnectionDetails<'_>,
        chained: Option<()>,
    ) -> Result<Option<Disguised>, ureq::Error> {
        let carrier = TcpConnector::default().connect(details, chained)?;

        Ok(carrier.map(|found| Disguised { carrier: found.boxed() }))
    }
}

impl Resolver for Local {
    fn resolve(
        &self,
        _uri: &Uri,
        _config: &Config,
        _timeout: NextTimeout,
    ) -> Result<ResolvedSocketAddrs, ureq::Error> {
        let mut addresses = self.empty();

        addresses.push(self.address);

        Ok(addresses)
    }
}

impl Server {
    fn finish(self) -> Vec<String> {
        self.is_finished.store(true, Ordering::SeqCst);

        self.handle.join().expect("the server ends")
    }

    fn start(scenes: Vec<Scene>) -> Self {
        assert!(!scenes.is_empty(), "a server plays at least one scene");

        let listener = TcpListener::bind("127.0.0.1:0").expect("a free port");
        let address = listener.local_addr().expect("a bound port");

        listener.set_nonblocking(true).expect("a polling listener");

        let is_finished = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&is_finished);
        let handle = thread::spawn(move || scenes_play(&listener, scenes, &flag));

        Self { address, handle, is_finished }
    }
}

impl Transport for Instant {
    fn fetch(&self, request: &Request, bytes_max: u64) -> Result<Response, Error> {
        assert!(bytes_max >= 1, "a body may hold at least one byte");

        self.agent.fetch(request, bytes_max)
    }

    fn stream<Sink>(&self, request: &Request, sink: &mut Sink) -> Result<Response, Error>
    where
        Sink: Seek + Write,
    {
        self.agent.stream(request, sink)
    }

    fn wait(&self, wait: &Wait<'_>) -> Result<(), Error> {
        assert!(!wait.reason.is_empty(), "a wait always says why");

        let mut waits = self
            .waits
            .lock()
            .map_err(|_poisoned| Error::Input("the wait log is poisoned".to_owned()))?;

        waits.push(wait.delay);
        drop(waits);

        Ok(())
    }
}

impl Seek for SinkRefusing {
    fn seek(&mut self, position: io::SeekFrom) -> io::Result<u64> {
        assert!(
            matches!(position, io::SeekFrom::Start(0) | io::SeekFrom::Current(0)),
            "a sink only ever stays where it began",
        );

        Ok(0)
    }
}

impl Write for SinkRefusing {
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        assert!(!bytes.is_empty(), "a sink is asked to hold something");

        Err(io::Error::other("the disk is full"))
    }
}

#[test]
fn a_bodiless_delete_names_json_and_sends_its_terms_escaped_while_a_read_names_nothing() {
    let server = Server::start(vec![answer(204, &[], ""), answer(200, &[], "{}")]);
    let client = client_of(&server);
    let called = DeleteLabelFromSpace { name: "a=b&id=1", space_key: "DEV" };
    let reply = client.send(&called).expect("a delete");
    let user = client.send(&GetCurrentUser).expect("a read");
    let seen = server.finish();
    let write = seen[0].to_lowercase();
    let read = seen[1].to_lowercase();

    assert!(reply.read().is_some(), "an empty answer reads as nothing");
    assert!(user.read().is_some(), "an object answer reads");

    assert!(write.starts_with("delete "), "the method survives: {write}");
    assert!(write.contains("content-type: application/json"), "a write says JSON: {write}");
    assert!(seen[0].contains("name=a%3Db%26id%3D1"), "the term is escaped: {}", seen[0]);
    assert!(seen[0].ends_with("\r\n\r\n"), "no body follows the headers: {}", seen[0]);
    assert!(read.starts_with("get "), "the read survives: {read}");
    assert!(!read.contains("content-type"), "a read names no type: {read}");
}

#[test]
fn a_body_cut_short_is_a_transport_failure() {
    let scene = said("HTTP/1.1 200 OK\r\nContent-Length: 100\r\nConnection: close\r\n\r\nshort");
    let server = Server::start(vec![scene]);
    let failure = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect_err("a failure");

    assert!(matches!(failure, Error::Transport(_)), "the wire failed: {failure:?}");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_body_past_its_limit_is_a_transport_failure() {
    let server = Server::start(vec![answer(200, &[], "12345")]);
    let failure = agent_of(&server).fetch(&request_read(), 4).expect_err("a refusal");

    assert!(matches!(failure, Error::Transport(_)), "the wire failed: {failure:?}");
    assert!(failure.message().contains("body read failed"), "the failure names the body");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_chunked_body_cut_short_is_a_transport_failure() {
    let scene = said(
        "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n5\r\nhello\r\n",
    );

    let server = Server::start(vec![scene]);
    let failure = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect_err("a failure");

    assert!(matches!(failure, Error::Transport(_)), "the wire failed: {failure:?}");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_chunked_body_reads_whole() {
    let scene = said(
        "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n\
         5\r\nhello\r\n6\r\n world\r\n0\r\n\r\n",
    );

    let server = Server::start(vec![scene]);
    let response = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect("an answer");

    assert_eq!(response.body, b"hello world", "each chunk arrives in order");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_connection_nobody_answers_is_a_transport_failure() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("a free port");
    let address = listener.local_addr().expect("a bound port");

    drop(listener);

    let agent = Agent::over(Loopback, Local { address });
    let failure = agent.fetch(&request_read(), BYTES_MAX).expect_err("a failure");

    assert!(matches!(failure, Error::Transport(_)), "the wire failed: {failure:?}");
}

#[test]
fn a_download_cut_short_starts_over_and_lands_whole() {
    let partial = said("HTTP/1.1 200 OK\r\nContent-Length: 90\r\nConnection: close\r\n\r\npartial");
    let server = Server::start(vec![partial, answer(200, &[], "the whole body")]);
    let client = client_of(&server);
    let mut sink = Cursor::new(Vec::new());

    let written = client.download(&DOWNLOAD, &mut sink).expect("a download");

    assert_eq!(written, 14, "the second attempt's length is the one reported");
    assert_eq!(sink.into_inner(), b"the whole body", "the sink holds the second attempt alone");
    assert_eq!(waits_of(&client).len(), 1, "one retry");
    assert_eq!(server.finish().len(), 2, "the download ran twice");
}

#[test]
fn a_download_lands_in_its_sink_and_never_in_memory() {
    let server = Server::start(vec![answer(200, &[], "the attachment")]);
    let mut sink = Cursor::new(Vec::new());
    let response = agent_of(&server).stream(&request_read(), &mut sink).expect("an answer");

    assert_eq!(response.streamed_count, 14, "every byte is counted");
    assert!(response.body.is_empty(), "nothing is held in memory");
    assert_eq!(sink.into_inner(), b"the attachment", "the sink holds the body");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_get_follows_a_redirect_and_leaves_its_credential_behind() {
    let elsewhere = "https://api.media.atlassian.com/file/1/binary";

    for target in [elsewhere, "/wiki/rest/api/moved"] {
        let moved = answer(302, &[("Location", target)], "");
        let server = Server::start(vec![moved, answer(200, &[], "the file")]);
        let response = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect("an answer");
        let seen = server.finish();

        assert_eq!(response.body, b"the file", "the redirect was followed to {target}");
        assert_eq!(seen.len(), 2, "two requests were made");
        assert!(seen[0].to_lowercase().contains("authorization: basic"), "the first is signed");
        assert!(!seen[1].to_lowercase().contains("authorization"), "no credential moves on");
    }
}

#[test]
fn a_rate_limit_waits_what_the_server_asked_for() {
    let limited = answer(429, &[("Retry-After", "7")], "");
    let server = Server::start(vec![limited, answer(200, &[], "{}")]);
    let client = client_of(&server);
    let reply = client.send(&GetCurrentUser).expect("the second attempt lands");
    let waits = waits_of(&client);

    assert!(reply.value().is_object(), "the reply is the second answer");
    assert_eq!(waits.len(), 1, "one wait");
    assert!(waits[0] >= Duration::from_secs(7), "the wait honors the server: {:?}", waits[0]);
    assert_eq!(server.finish().len(), 2, "the request ran twice");
}

#[test]
fn a_redirect_loop_stops_at_its_budget() {
    let scenes = vec![answer(302, &[("Location", "/again")], ""); 12];
    let server = Server::start(scenes);
    let failure = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect_err("a failure");
    let seen = server.finish();

    assert!(matches!(failure, Error::Transport(_)), "a loop is a failure: {failure:?}");
    assert!(failure.message().contains("too many redirects"), "the failure says why");

    assert_eq!(
        u32::try_from(seen.len()).ok(),
        REDIRECT_COUNT_MAX.checked_add(1),
        "the first request and its budget of redirects",
    );
}

#[test]
fn a_redirect_to_plain_http_is_refused_before_it_connects() {
    let moved = answer(302, &[("Location", "http://api.atlassian.com/plain")], "");
    let server = Server::start(vec![moved, answer(200, &[], "never sent")]);
    let failure = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect_err("a refusal");

    assert!(matches!(failure, Error::Transport(_)), "a refusal: {failure:?}");
    assert!(failure.message().contains("https only"), "the failure says why");
    assert_eq!(server.finish().len(), 1, "the plain address was never reached");
}

#[test]
fn a_refusal_is_read_rather_than_raised() {
    for status in [400, 401, 404, 429, 500, 503] {
        let server = Server::start(vec![answer(status, &[], "refused")]);
        let response = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect("an answer");

        assert_eq!(response.status, status, "the status arrives");
        assert_eq!(response.body, b"refused", "the body of a refusal arrives");
        assert_eq!(server.finish().len(), 1, "one request was made");
    }
}

#[test]
fn a_refused_download_cut_short_keeps_its_status() {
    let partial =
        said("HTTP/1.1 404 Not Found\r\nContent-Length: 90\r\nConnection: close\r\n\r\ngone");

    let server = Server::start(vec![partial]);
    let mut sink = Cursor::new(Vec::new());
    let response = agent_of(&server).stream(&request_read(), &mut sink).expect("the status came");
    let text = String::from_utf8_lossy(&response.body).into_owned();

    assert_eq!(response.status, 404, "a lost body never hides the refusal");
    assert!(text.starts_with("gone"), "what did arrive is kept: {text}");
    assert!(text.contains("the rest was lost"), "the message says the body was cut: {text}");
    assert!(sink.into_inner().is_empty(), "an error page never lands in a file");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_refused_download_holds_no_more_than_its_excerpt() {
    let page = "x".repeat(4_096);
    let server = Server::start(vec![answer(404, &[], &page)]);
    let mut sink = Cursor::new(Vec::new());
    let response = agent_of(&server).stream(&request_read(), &mut sink).expect("an answer");

    assert_eq!(response.status, 404, "a long error page keeps its status");
    assert_eq!(size::of_slice(&response.body), limits::ERROR_EXCERPT_BYTES_MAX, "a bounded read");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_refused_download_keeps_what_confluence_said() {
    let said = "{\"statusCode\":404,\"message\":\"no such attachment\"}";
    let server = Server::start(vec![answer(404, &[], said)]);
    let client = client_of(&server);
    let mut sink = Cursor::new(Vec::new());
    let failure = client.download(&DOWNLOAD, &mut sink).expect_err("a refusal");
    let is_refused = matches!(failure, Error::Confluence { status: 404, message: _ });

    assert!(is_refused, "refused: {failure:?}");
    assert!(failure.message().contains("no such attachment"), "the reason survives: {failure:?}");
    assert!(sink.into_inner().is_empty(), "an error page never lands in a file");
    assert_eq!(server.finish().len(), 1, "a refusal is not retried");
}

#[test]
fn a_refused_download_writes_nothing() {
    let server = Server::start(vec![answer(404, &[], "the error page")]);
    let mut sink = Cursor::new(Vec::new());
    let response = agent_of(&server).stream(&request_read(), &mut sink).expect("an answer");

    assert_eq!(response.status, 404, "a refusal is read");
    assert_eq!(response.streamed_count, 0, "nothing is counted");
    assert!(sink.into_inner().is_empty(), "an error page never lands in a file");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

#[test]
fn a_server_error_is_read_again_until_it_answers() {
    let failing = answer(503, &[], "busy");
    let server = Server::start(vec![failing.clone(), failing, answer(200, &[], "{}")]);
    let client = client_of(&server);
    let reply = client.send(&GetCurrentUser).expect("the third attempt lands");

    assert!(reply.value().is_object(), "the reply is the last answer");
    assert_eq!(waits_of(&client).len(), 2, "two waits");
    assert_eq!(server.finish().len(), 3, "the read ran three times");
}

#[test]
fn a_sink_that_refuses_a_download_is_not_a_network_failure() {
    let server = Server::start(vec![answer(200, &[], "the attachment")]);
    let client = client_of(&server);
    let failure = client.download(&DOWNLOAD, &mut SinkRefusing).expect_err("a refusal");

    assert!(matches!(failure, Error::Input(_)), "the disk is not the wire: {failure:?}");
    assert!(waits_of(&client).is_empty(), "no retry");
    assert_eq!(server.finish().len(), 1, "a full disk is never asked twice");
}

#[test]
fn a_timeout_ends_a_call_that_never_answers() {
    let server = Server::start(vec![Scene::Hang]);
    let agent = agent_of(&server);
    let failure = agent.call(&request_read(), Duration::from_millis(250)).expect_err("a timeout");

    assert!(matches!(failure, Error::Transport(_)), "the wire failed: {failure:?}");
    assert!(failure.message().contains("timeout"), "the failure says why");
    assert_eq!(server.finish().len(), 1, "the request arrived and was never answered");
}

#[test]
fn a_walk_crosses_its_pages_over_the_wire() {
    let next = "/wiki/api/v2/spaces?limit=1&cursor=c%2F2";
    let first = json!({"_links": {"next": next}, "results": [{"key": "A"}]}).to_string();
    let second = json!({"_links": {}, "results": [{"key": "B"}]}).to_string();
    let server = Server::start(vec![answer(200, &[], &first), answer(200, &[], &second)]);
    let client = client_of(&server);

    let pages: Vec<_> = client.walk(&GetSpaces).map(|page| page.expect("a page").count()).collect();

    let seen = server.finish();

    assert_eq!(pages, [1, 1], "each page holds one space");
    assert!(seen[1].contains("cursor=c%2F2"), "the second page follows the link: {}", seen[1]);
}

#[test]
fn a_write_keeps_a_redirect_for_its_caller() {
    let moved = answer(303, &[("Location", "https://api.atlassian.com/task/1")], "");
    let server = Server::start(vec![moved, answer(200, &[], "never sent")]);
    let mut request = request_write();

    request.body_json(&json!({"title": "moved"})).expect("a body");

    let response = agent_of(&server).fetch(&request, BYTES_MAX).expect("an answer");

    assert_eq!(response.status, 303, "a write is not redirected");
    assert_eq!(response.location.as_deref(), Some("https://api.atlassian.com/task/1"), "kept");
    assert_eq!(server.finish().len(), 1, "the address was never followed");
}

#[test]
fn a_write_that_lost_its_connection_is_never_sent_again() {
    let server = Server::start(vec![Scene::Vanish, answer(201, &[], "{}")]);
    let client = client_of(&server);
    let body = json!({"spaceId": "1", "title": "once"});
    let failure = client.send(&CreatePage { body: &body }).expect_err("an unknown outcome");

    assert!(matches!(failure, Error::Indefinite(_)), "it may have landed: {failure:?}");
    assert!(waits_of(&client).is_empty(), "no retry was planned");
    assert_eq!(server.finish().len(), 1, "a write that may have landed is sent once");
}

fn agent_of(server: &Server) -> Agent {
    assert_ne!(server.address.port(), 0, "a server is bound");

    Agent::over(Loopback, Local { address: server.address })
}

#[test]
fn an_attachment_upload_reaches_the_wire_as_one_multipart_body() {
    let reply = json!({"results": [{"id": "att1"}], "size": 1_i64}).to_string();
    let server = Server::start(vec![answer(200, &[], &reply)]);
    let client = client_of(&server);

    let part = Part {
        bytes: b"attachment text",
        content_type: "text/plain",
        file_name: Some("notes.txt"),
        name: "file",
    };

    let parts = [part];
    let found = client.send(&CreateAttachment { body: &parts, id: "123" });
    let seen = server.finish();
    let request = seen[0].to_lowercase();

    assert!(found.is_ok(), "the upload lands: {found:?}");
    assert!(request.contains("x-atlassian-token: no-check"), "the check is waived");
    assert!(request.contains("content-type: multipart/form-data; boundary="), "a form");
    assert!(request.contains("filename=\"notes.txt\""), "the part names its file");
    assert!(request.contains("attachment text"), "the bytes travel");
}

#[test]
fn an_entity_tag_reaches_the_wire_and_a_not_modified_answer_reads_as_its_status() {
    let server = Server::start(vec![answer(304, &[("ETag", "\"tag\"")], "")]);
    let client = client_of(&server);
    let filter = [("status", "done")];

    let options = get_database_entries::Options {
        cursor: None,
        filter: Some(&filter),
        if_none_match: Some("\"tag\""),
        limit: None,
        sort: None,
    };

    let called = GetDatabaseEntries { id: "7" };
    let failure = client.send_with(&called, &options).expect_err("nothing new to read");
    let seen = server.finish();
    let request = seen[0].to_lowercase();

    assert_eq!(failure.status(), Some(304), "the caller learns nothing changed: {failure:?}");
    assert!(request.contains("if-none-match: \"tag\""), "the tag travels: {request}");
    assert!(seen[0].contains("filter%5Bstatus%5D=done"), "the deep object travels: {}", seen[0]);
    assert!(waits_of(&client).is_empty(), "a 304 is an answer, not a failure to retry");
}

fn answer(status: u16, headers: &[(&str, &str)], body: &str) -> Scene {
    assert!((100..=599).contains(&status), "a scene answers with a real status");

    let mut text = format!(
        "HTTP/1.1 {status} Scene\r\nConnection: close\r\nContent-Length: {}\r\n",
        body.len(),
    );

    for (name, value) in headers {
        text.push_str(name);
        text.push_str(": ");
        text.push_str(value);
        text.push_str("\r\n");
    }

    text.push_str("\r\n");
    text.push_str(body);

    Scene::Say(text.into_bytes())
}

fn client_of(server: &Server) -> Client<Instant> {
    let network = Instant { agent: agent_of(server), waits: Mutex::new(Vec::new()) };

    Client::new(network, &CREDENTIALS).expect("the fixture credentials")
}

fn connection_accept(listener: &TcpListener, is_finished: &AtomicBool) -> Option<TcpStream> {
    for _poll_index in 0..POLL_COUNT_MAX {
        match listener.accept() {
            Ok((stream, _peer)) => {
                stream.set_nonblocking(false).expect("a blocking stream");

                return Some(stream);
            }
            Err(cause) => {
                assert_eq!(cause.kind(), io::ErrorKind::WouldBlock, "a poll only ever waits");
            }
        }

        if is_finished.load(Ordering::SeqCst) {
            return None;
        }

        thread::park_timeout(Duration::from_millis(POLL_MILLIS));
    }

    None
}

#[test]
fn every_signal_of_a_rate_limit_survives_the_wire() {
    let headers = [
        ("Retry-After", "31"),
        ("RateLimit-Reason", "confluence-quota-tenant-based"),
        ("X-RateLimit-NearLimit", "true"),
    ];

    let server = Server::start(vec![answer(429, &headers, "")]);
    let response = agent_of(&server).fetch(&request_read(), BYTES_MAX).expect("an answer");
    let reason = response.rate_limit_reason.as_deref();

    assert_eq!(response.status, 429, "the status");
    assert_eq!(response.retry_after_seconds, Some(31), "the delay");
    assert_eq!(reason, Some("confluence-quota-tenant-based"), "the reason");
    assert!(response.is_near_limit, "the flag");
    assert_eq!(server.finish().len(), 1, "one request was made");
}

fn request_arrived(bytes: &[u8]) -> bool {
    let text = String::from_utf8_lossy(bytes);

    let Some((head, body)) = text.split_once("\r\n\r\n") else {
        return false;
    };

    let length = head.lines().find_map(|line| {
        let lowered = line.to_lowercase();

        lowered.strip_prefix("content-length: ")?.trim().parse::<usize>().ok()
    });

    assert!(head.contains("HTTP/1.1"), "a request names its protocol");

    body.len() >= length.unwrap_or_default()
}

fn request_read() -> Request {
    let request = Request::of::<GetCurrentUser>(URL_READ.to_owned(), AUTHORIZATION);

    assert!(request.is_read(), "a fixture read is a read");

    request
}

fn request_take(stream: &mut TcpStream) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    let mut chunk = [0_u8; 4_096];

    for _read_index in 0..READ_COUNT_MAX {
        if request_arrived(&bytes) {
            break;
        }

        let count = stream.read(&mut chunk).unwrap_or_default();

        if count == 0 {
            break;
        }

        bytes.extend_from_slice(&chunk[..count]);
    }

    String::from_utf8_lossy(&bytes).into_owned()
}

fn request_write() -> Request {
    let body = json!({"title": "fixture"});
    let operation = CreatePage { body: &body };
    let request = Request::of::<CreatePage<'_>>(URL_READ.to_owned(), AUTHORIZATION);

    assert!(!request.is_read(), "a fixture write is a write");
    assert!(format!("{operation:?}").contains("CreatePage"), "a fixture names its operation");

    request
}

fn said(text: &str) -> Scene {
    assert!(text.starts_with("HTTP/1.1 "), "a scene speaks HTTP");

    Scene::Say(text.as_bytes().to_vec())
}

fn scene_play(mut stream: TcpStream, scene: Scene) -> String {
    let timeout = Some(Duration::from_secs(READ_TIMEOUT_SECONDS));

    stream.set_read_timeout(timeout).expect("a read timeout");

    let request = request_take(&mut stream);

    match scene {
        Scene::Hang => {
            let _read = stream.read(&mut [0_u8; 1]);
        }
        Scene::Say(bytes) => {
            let _written = stream.write_all(&bytes);
        }
        Scene::Vanish => {}
    }

    assert!(request.contains("HTTP/1.1"), "a scene plays to a request: {request}");

    request
}

fn scenes_play(
    listener: &TcpListener,
    scenes: Vec<Scene>,
    is_finished: &AtomicBool,
) -> Vec<String> {
    assert!(!scenes.is_empty(), "a server plays at least one scene");

    let mut seen = Vec::with_capacity(scenes.len());

    for scene in scenes {
        let Some(stream) = connection_accept(listener, is_finished) else {
            break;
        };

        seen.push(scene_play(stream, scene));
    }

    seen
}

#[test]
fn the_request_carries_everything_the_operation_put_in_it() {
    let server = Server::start(vec![answer(201, &[], "{}")]);
    let mut request = request_write();

    request.body_json(&json!({"title": "hello"})).expect("a body");

    let response = agent_of(&server).fetch(&request, BYTES_MAX).expect("an answer");
    let seen = server.finish();
    let head = seen[0].to_lowercase();

    assert_eq!(response.status, 201, "the answer arrives");
    assert!(seen[0].starts_with(LINE_WRITE), "the request line");
    assert!(head.contains("host: api.atlassian.com"), "the host");
    assert!(head.contains(&format!("authorization: {}", AUTHORIZATION.to_lowercase())), "signed");
    assert!(head.contains("accept: application/json"), "the accept header");
    assert!(head.contains("content-type: application/json"), "the content type");
    assert!(seen[0].contains("\"title\":\"hello\""), "the body travels");
}

fn waits_of(client: &Client<Instant>) -> Vec<Duration> {
    let waits = client.network().waits.lock().expect("the waits").clone();

    assert!(waits.len() <= 16, "a test waits a few times");

    waits
}
