use std::io::{self, Seek, Write};
use std::process::ExitCode;

use confluence::v2::shape::{
    BodySingle,
    BodyType,
    CreateFooterCommentReply,
    GetPageByIDReply,
    SpaceBulk,
};
use confluence::v2::{CreateFooterComment, GetPageByID, GetSpaces, get_page_by_id};
use confluence::{Client, Credentials, Error, Method, Request, Response, Transport, Wait, adf};
use serde_json::{Value, json};

#[derive(Debug)]
struct Offline {
    page: Value,
    spaces: Value,
}

impl Offline {
    fn new() -> Self {
        let document = json!({
            "type": "doc",
            "version": 1_i64,
            "content": [{
                "type": "paragraph",
                "content": [{"type": "text", "text": "The login times out."}],
            }],
        });

        let page = json!({
            "body": {
                "atlas_doc_format": {
                    "representation": "atlas_doc_format",
                    "value": document.to_string(),
                },
            },
            "id": "101",
            "title": "Checkout stalls",
        });

        let spaces = json!({
            "_links": {"base": "https://example.atlassian.net/wiki"},
            "results": [{"key": "DEMO"}, {"key": "SHOP"}],
        });

        assert!(page.get("title").is_some(), "the page carries a title");
        assert!(spaces.get("results").is_some(), "the page of spaces carries its spaces");

        Self { page, spaces }
    }
}

impl Transport for Offline {
    fn fetch(&self, request: &Request, bytes_max: u64) -> Result<Response, Error> {
        assert!(bytes_max >= 1, "a body may hold at least one byte");
        assert!(request.url().starts_with("https://"), "every request is encrypted");

        let (status, body) = if request.method() == Method::Post {
            (200, json!({"id": "9"}))
        } else if request.url().contains("/wiki/api/v2/spaces") {
            (200, self.spaces.clone())
        } else {
            (200, self.page.clone())
        };

        Ok(Response {
            body: serde_json::to_vec(&body).unwrap_or_default(),
            is_near_limit: false,
            location: None,
            rate_limit_reason: None,
            retry_after_seconds: None,
            status,
            streamed_count: 0,
        })
    }

    fn stream<Sink>(&self, request: &Request, sink: &mut Sink) -> Result<Response, Error>
    where
        Sink: Seek + Write,
    {
        assert!(request.url().starts_with("https://"), "every request is encrypted");

        let response = self.fetch(request, 1_024)?;

        sink.write_all(&response.body)
            .map_err(|cause| Error::Input(format!("cannot write the download: {cause}")))?;

        assert!(response.status >= 200, "the example only ever succeeds");

        Ok(response)
    }

    fn wait(&self, wait: &Wait<'_>) -> Result<(), Error> {
        assert!(!wait.reason.is_empty(), "a wait says why");
        assert!(self.spaces.is_object(), "the example holds its fixtures");

        Ok(())
    }
}

fn main() -> ExitCode {
    let Err(failure) = run() else {
        return ExitCode::SUCCESS;
    };

    assert!(!failure.message().is_empty(), "a failure says why");
    assert!(failure.status().is_none(), "the example only ever succeeds");

    if writeln!(io::stderr().lock(), "{failure}").is_err() {
        return ExitCode::from(2);
    }

    ExitCode::FAILURE
}

fn report(line: &str) -> Result<(), Error> {
    assert!(!line.is_empty(), "a report says something");
    assert!(!line.contains('\n'), "a report is one line");

    writeln!(io::stdout().lock(), "{line}")
        .map_err(|cause| Error::Input(format!("cannot write to standard output: {cause}")))
}

fn run() -> Result<(), Error> {
    let credentials = Credentials::Token {
        cloud_id: "11111111-2222-3333-4444-555555555555",
        email: "user@example.com",
        token: "token",
    };

    let options = get_page_by_id::Options {
        body_format: Some("atlas_doc_format"),
        get_draft: None,
        include_collaborators: None,
        include_direct_children: None,
        include_favorited_by_user_status_current: None,
        include_labels: None,
        include_likes: None,
        include_operations: None,
        include_properties: None,
        include_version: None,
        include_versions: None,
        include_webresources: None,
        status: None,
        version: None,
    };

    let client = Client::new(Offline::new(), &credentials)?;
    let reply = client.send_with(&GetPageByID { id: 101 }, &options)?;
    let page = reply.read().ok_or_else(|| Error::Reply("the page is no object".to_owned()))?;
    let held = page.member(GetPageByIDReply::BODY);
    let format = held.and_then(|found| found.member(BodySingle::ATLAS_DOC_FORMAT));
    let stored = format.and_then(|found| found.member(BodyType::VALUE)).unwrap_or_default();

    let document: Value = serde_json::from_str(stored)
        .map_err(|cause| Error::Reply(format!("the page body is no document: {cause}")))?;

    report(page.member(GetPageByIDReply::TITLE).unwrap_or("no title"))?;
    report(&adf::text_read(&document))?;

    for walked in client.walk(&GetSpaces) {
        let found = walked?;

        assert!(found.count() >= 1, "the example holds spaces");

        for space in found.items() {
            report(space.member(SpaceBulk::KEY).unwrap_or("no key"))?;
        }
    }

    let body = json!({
        "body": {"representation": "storage", "value": "<p>Seen.</p>"},
        "pageId": "101",
    });

    let created = client.send(&CreateFooterComment { body: &body })?;
    let identifier = created.read().and_then(|found| found.member(CreateFooterCommentReply::ID));

    assert!(identifier.is_some(), "the example reply names the comment");

    report(identifier.unwrap_or("no id"))
}
