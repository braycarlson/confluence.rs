use std::env;
use std::fs;
use std::io::{self, Cursor, Write as _};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use confluence::v1::shape::{
    Content,
    ContentArray,
    ContentRestriction,
    GetRestrictionsByOperationReply,
    User,
};
use confluence::v1::{
    CreateAttachment,
    DownloadAttatchment,
    GetCurrentUser,
    GetDescendantsOfType,
    GetRestrictionsByOperation,
    SearchByCQL,
    get_current_user,
    get_descendants_of_type,
    search_by_cql,
};
use confluence::v2::shape::{
    CreatePageReply,
    GetPageByIDReply,
    GetPageByIDReplyLinks,
    GetSpacesReply,
    MultiEntityLinks,
    SpaceBulk,
};
use confluence::v2::{
    CreateFooterComment,
    CreatePage,
    DeletePage,
    GetPageByID,
    GetPageFooterComments,
    GetSpaces,
    delete_page,
    get_page_by_id,
    get_page_footer_comments,
    get_spaces,
};
use confluence::{Agent, AttachmentID, Client, Credentials, Error, PageID, Part, SpaceKey, adf};
use serde_json::{Value, json};

const ACCOUNT_FILE: &str = "test-account.json";
const ACCOUNT_FIELDS: [&str; 5] = ["cloud_id", "email", "site", "space_key", "token"];
const ATTACHMENT_BYTES: &[u8] = b"a body written by the live check";
const COMMENT_COUNT: u64 = 3;
const PAGE_TEXT: &str = "A page written by the confluence.rs live check";
const SEARCH_PAGE_COUNT_MAX: u8 = 3;
const SPACE_KEY_ABSENT: &str = "CONFLUENCERSABSENT";
const SPACE_PAGE_COUNT_MAX: u8 = 5;
const USER_EXPANSIONS: [&str; 2] = ["personalSpace", "operations"];
const TITLE: &str = "A disposable page from the confluence.rs live check";

#[derive(Debug)]
struct Account {
    settings: Value,
    site: String,
    space_key: SpaceKey,
}

fn main() -> ExitCode {
    let Err(failure) = run() else {
        return ExitCode::SUCCESS;
    };

    assert!(!failure.message().is_empty(), "a failure says why");

    if writeln!(io::stderr().lock(), "live check failed: {failure}").is_err() {
        return ExitCode::from(2);
    }

    ExitCode::FAILURE
}

fn make_page(client: &Client<Agent>, space_id: &str) -> Result<PageID, Error> {
    assert!(!space_id.is_empty(), "a page lands in a space");

    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |since| since.as_secs());

    let body = json!({
        "body": {"representation": "storage", "value": format!("<p>{PAGE_TEXT}</p>")},
        "spaceId": space_id,
        "status": "current",
        "title": format!("{TITLE} {stamp}"),
    });

    let reply = client.send(&CreatePage { body: &body })?;
    let created = reply.read().ok_or_else(|| reply_missing("the created page"))?;
    let id = created.member(CreatePageReply::ID).ok_or_else(|| reply_missing("the page id"))?;
    let parsed = id.parse().ok().and_then(PageID::new);
    let page = parsed.ok_or_else(|| Error::Reply(format!("the page id {id} is no number")))?;

    report(&format!("page {page} created"))?;

    Ok(page)
}

fn read_account() -> Result<Account, Error> {
    let path = env::args().nth(1).unwrap_or_else(|| ACCOUNT_FILE.to_owned());

    assert!(!path.is_empty(), "a live check names its credentials");

    let text = fs::read_to_string(&path)
        .map_err(|cause| Error::Input(format!("cannot read {path}: {cause}")))?;

    let settings: Value = serde_json::from_str(&text)
        .map_err(|cause| Error::Input(format!("cannot parse {path}: {cause}")))?;

    for field in ACCOUNT_FIELDS {
        if text_of(&settings, field).is_empty() {
            return Err(Error::Input(format!("{path} names no {field}")));
        }
    }

    let key = text_of(&settings, "space_key");
    let space_key = SpaceKey::parse(key).ok_or_else(|| Error::Input(format!("{key} is no key")))?;
    let site = text_of(&settings, "site").trim_end_matches('/').to_owned();

    assert!(!space_key.text().is_empty(), "a checked space key is present");
    assert!(!site.ends_with('/'), "a site compares without its last slash");

    Ok(Account { settings, site, space_key })
}

fn reply_missing(what: &str) -> Error {
    assert!(!what.is_empty(), "a missing reply names what is missing");

    let failure = Error::Reply(format!("the reply holds no {what}"));

    assert!(matches!(failure, Error::Reply(_)), "a missing member is a fault of the reply");

    failure
}

fn report(line: &str) -> Result<(), Error> {
    assert!(!line.is_empty(), "a report says something");
    assert!(!line.contains('\n'), "a report is one line");

    writeln!(io::stdout().lock(), "{line}")
        .map_err(|cause| Error::Input(format!("cannot write to standard output: {cause}")))
}

fn run() -> Result<(), Error> {
    let account = read_account()?;

    let credentials = Credentials::Token {
        cloud_id: text_of(&account.settings, "cloud_id"),
        email: text_of(&account.settings, "email"),
        token: text_of(&account.settings, "token"),
    };

    let client = Client::new(Agent::new(), &credentials)?;

    assert!(!account.site.is_empty(), "a live check names the site it expects");
    assert!(!account.space_key.text().is_empty(), "a live check names its space");

    let space_id = verify_site(&client, &account)?;

    verify_space_keys(&client, &account.space_key)?;
    verify_reads(&client)?;

    let page = make_page(&client, &space_id)?;
    let operation = DeletePage { id: page.get() };
    let steps = verify_page(&client, page, &account.site);
    let deleted = client.send(&operation);
    let purge = delete_page::Options { draft: None, purge: Some(true) };
    let purged = deleted.and_then(|_trashed| client.send_with(&operation, &purge));

    steps?;

    let _reply = purged?;

    report(&format!("page {page} deleted and purged"))
}

fn text_of<'value>(settings: &'value Value, field: &str) -> &'value str {
    assert!(!field.is_empty(), "a setting is named");

    let text = settings.get(field).and_then(Value::as_str).unwrap_or_default();

    assert!(!text.contains('\n'), "a setting is one line");

    text
}

fn verify_attachment(client: &Client<Agent>, page: PageID) -> Result<(), Error> {
    assert!(page.get() >= 1, "an attachment lands on a page");

    let parts = [Part {
        bytes: ATTACHMENT_BYTES,
        content_type: "text/plain",
        file_name: Some("live.txt"),
        name: "file",
    }];

    let id = page.to_string();
    let reply = client.send(&CreateAttachment { body: &parts, id: &id })?;
    let uploaded = reply.read().and_then(|array| array.member(ContentArray::RESULTS));
    let first = uploaded.and_then(|mut found| found.next());
    let attachment = first.ok_or_else(|| reply_missing("an attachment"))?;
    let text = attachment.member(Content::ID).ok_or_else(|| reply_missing("an attachment id"))?;
    let parsed = AttachmentID::parse(text);
    let handle = parsed.ok_or_else(|| Error::Reply(format!("{text} is no attachment id")))?;
    let operation = DownloadAttatchment { attachment_id: handle.text(), id: &id };
    let mut sink = Cursor::new(Vec::new());
    let written = client.download(&operation, &mut sink)?;

    assert_eq!(sink.get_ref().as_slice(), ATTACHMENT_BYTES, "a download returns what was uploaded");
    assert_eq!(written, 32, "a download counts what it wrote");

    report("attachment uploaded and downloaded through its redirect")
}

fn verify_comment_walk(client: &Client<Agent>, page: PageID) -> Result<(), Error> {
    assert!(page.get() >= 1, "a thread belongs to a page");

    let body = json!({
        "body": {"representation": "storage", "value": "<p>A comment from the live check</p>"},
        "pageId": page.to_string(),
    });

    for _comment_index in 0..COMMENT_COUNT {
        let _reply = client.send(&CreateFooterComment { body: &body })?;
    }

    let options = get_page_footer_comments::Options {
        body_format: None,
        cursor: None,
        limit: Some(1),
        sort: Some("created-date"),
        status: None,
    };

    let operation = GetPageFooterComments { id: page.get() };
    let mut page_count = 0_u64;
    let mut ids: Vec<String> = Vec::new();

    for walked in client.walk_with(&operation, options) {
        for value in walked?.into_values() {
            ids.push(value.get("id").and_then(Value::as_str).unwrap_or_default().to_owned());
        }

        page_count = page_count.strict_add(1);
    }

    ids.sort_unstable();
    ids.dedup();

    assert!(page_count >= COMMENT_COUNT, "a page of one comment holds at most one");
    assert_eq!(u64::try_from(ids.len()).ok(), Some(COMMENT_COUNT), "each comment arrives once");

    report(&format!("a link walk crossed {page_count} pages of one comment"))
}

fn verify_descendant_walk(client: &Client<Agent>, page: PageID) -> Result<(), Error> {
    assert!(page.get() >= 1, "a descendant belongs to a page");

    let id = page.to_string();
    let operation = GetDescendantsOfType { id: &id, kind: "comment" };

    let options =
        get_descendants_of_type::Options { depth: None, expand: None, limit: Some(1), start: None };

    let mut item_count = 0_u64;
    let mut page_count = 0_u64;

    for walked in client.walk_with(&operation, options) {
        item_count = item_count.strict_add(walked?.count());
        page_count = page_count.strict_add(1);
    }

    assert_eq!(item_count, COMMENT_COUNT, "every comment is a descendant of its page");
    assert!(page_count >= COMMENT_COUNT, "a page of one comment holds at most one");

    report(&format!("a start walk crossed {page_count} pages of one comment"))
}

fn verify_page(client: &Client<Agent>, page: PageID, site: &str) -> Result<(), Error> {
    assert!(page.get() >= 1, "the steps run on one page");
    assert!(!site.ends_with('/'), "a site compares without its last slash");

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

    let reply = client.send_with(&GetPageByID { id: page.get() }, &options)?;
    let stored = reply.value().pointer("/body/atlas_doc_format/value").and_then(Value::as_str);

    let document: Value = serde_json::from_str(stored.unwrap_or_default())
        .map_err(|cause| Error::Reply(format!("the page body is no document: {cause}")))?;

    let read = reply.read().ok_or_else(|| reply_missing("the page"))?;
    let links = read.member(GetPageByIDReply::LINKS).ok_or_else(|| reply_missing("links"))?;
    let base = links.member(GetPageByIDReplyLinks::BASE).unwrap_or_default();
    let webui = links.member(GetPageByIDReplyLinks::WEBUI).unwrap_or_default();
    let expected = format!("{site}/wiki");

    assert_eq!(adf::text_read(&document), PAGE_TEXT, "the page reads back as prose");
    assert_eq!(base.trim_end_matches('/'), expected, "the reply adds the site to the page links");
    assert!(!webui.is_empty(), "the page keeps its own links beside the base");

    report("page read back, flattened, and linked to its site")?;
    verify_comment_walk(client, page)?;
    verify_descendant_walk(client, page)?;
    verify_restrictions(client, page)?;
    verify_attachment(client, page)
}

fn verify_reads(client: &Client<Agent>) -> Result<(), Error> {
    let expansions = get_current_user::Options { expand: Some(&USER_EXPANSIONS) };
    let user = client.send_with(&GetCurrentUser, &expansions)?;
    let account = user.read().and_then(|found| found.member(User::ACCOUNT_ID));
    let operations = user.value().get("operations").is_some_and(Value::is_array);

    let options = get_spaces::Options {
        cursor: None,
        description_format: None,
        favorited_by: None,
        ids: None,
        include_icon: None,
        keys: None,
        kind: None,
        labels: None,
        limit: Some(1),
        not_favorited_by: None,
        sort: None,
        status: None,
    };

    let mut space_count = 0_u64;
    let mut page_count = 0_u64;

    for walked in client.walk_with(&GetSpaces, options).take(usize::from(SPACE_PAGE_COUNT_MAX)) {
        space_count = space_count.strict_add(walked?.count());
        page_count = page_count.strict_add(1);
    }

    assert!(account.is_some(), "the current user has an account id");
    assert!(operations, "a comma list expands each value it names, the last one included");
    assert!(space_count <= page_count, "a page of one space holds at most one");

    report(&format!("reads: a link walk crossed {page_count} pages of one space"))?;
    verify_search_walk(client)
}

fn verify_restrictions(client: &Client<Agent>, page: PageID) -> Result<(), Error> {
    assert!(page.get() >= 1, "a restriction belongs to a page");

    let id = page.to_string();
    let reply = client.send(&GetRestrictionsByOperation { id: &id })?;
    let read = reply.read().ok_or_else(|| reply_missing("the restrictions"))?;
    let fields = [GetRestrictionsByOperationReply::READ, GetRestrictionsByOperationReply::UPDATE];
    let operation = ContentRestriction::OPERATION;
    let entries = fields.map(|field| read.member(field));
    let operations = entries.map(|entry| entry?.member(operation));

    assert_eq!(operations, [Some("read"), Some("update")], "each operation keys its own entry");

    report("restrictions read back keyed by operation")
}

fn verify_search_walk(client: &Client<Agent>) -> Result<(), Error> {
    let options = search_by_cql::Options {
        cache_buster: None,
        cqlcontext: None,
        cursor: None,
        excerpt: None,
        exclude_spaces_current: None,
        expand: None,
        include_archived_spaces: None,
        limit: Some(1),
        next: None,
        previous: None,
        site_permission_type_filter: None,
        start: None,
    };

    let operation = SearchByCQL { cql: "type = space" };
    let mut results: Vec<String> = Vec::new();
    let mut page_count = 0_u64;

    for walked in client.walk_with(&operation, options).take(usize::from(SEARCH_PAGE_COUNT_MAX)) {
        results.extend(walked?.into_values().iter().map(Value::to_string));
        page_count = page_count.strict_add(1);
    }

    let item_count = results.len();

    results.sort_unstable();
    results.dedup();

    assert_eq!(results.len(), item_count, "a cursor walk hands back each result once");

    report(&format!("a cursor walk crossed {page_count} pages and found {item_count} spaces"))
}

fn verify_site(client: &Client<Agent>, account: &Account) -> Result<String, Error> {
    assert!(!account.site.is_empty(), "a live check names the site it expects");

    let keys = [account.space_key.text()];

    let options = get_spaces::Options {
        cursor: None,
        description_format: None,
        favorited_by: None,
        ids: None,
        include_icon: None,
        keys: Some(&keys),
        kind: None,
        labels: None,
        limit: None,
        not_favorited_by: None,
        sort: None,
        status: None,
    };

    let reply = client.send_with(&GetSpaces, &options)?;
    let spaces = reply.read().ok_or_else(|| reply_missing("the space list"))?;
    let links = spaces.member(GetSpacesReply::LINKS);
    let base = links.and_then(|found| found.member(MultiEntityLinks::BASE)).unwrap_or_default();
    let expected = format!("{}/wiki", account.site);

    if base.trim_end_matches('/') != expected {
        return Err(Error::Input(format!(
            "these credentials reach {base}, not {expected}, so nothing else runs",
        )));
    }

    let first = spaces.member(GetSpacesReply::RESULTS).and_then(|mut found| found.next());
    let absent = Error::Input(format!("the site holds no space {}", account.space_key));
    let space = first.ok_or(absent)?;
    let id = space.member(SpaceBulk::ID).ok_or_else(|| reply_missing("the space id"))?;

    report(&format!("site confirmed: {base}"))?;

    Ok(id.to_owned())
}

fn verify_space_keys(client: &Client<Agent>, key: &SpaceKey) -> Result<(), Error> {
    assert!(!key.text().is_empty(), "a filter names a space");
    assert_ne!(key.text(), SPACE_KEY_ABSENT, "the absent key names no real space");

    for keys in [[key.text(), SPACE_KEY_ABSENT], [SPACE_KEY_ABSENT, key.text()]] {
        let options = get_spaces::Options {
            cursor: None,
            description_format: None,
            favorited_by: None,
            ids: None,
            include_icon: None,
            keys: Some(&keys),
            kind: None,
            labels: None,
            limit: None,
            not_favorited_by: None,
            sort: None,
            status: None,
        };

        let reply = client.send_with(&GetSpaces, &options)?;
        let spaces = reply.read().and_then(|found| found.member(GetSpacesReply::RESULTS));
        let listed = spaces.into_iter().flatten();
        let found: Vec<&str> = listed.filter_map(|space| space.member(SpaceBulk::KEY)).collect();

        assert_eq!(found, [key.text()], "two keys filter as repeated terms in either order");
    }

    report("a list of two keys filtered as repeated terms in either order")
}
