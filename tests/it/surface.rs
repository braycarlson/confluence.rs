use std::io::Cursor;

use confluence::automation::SearchManualRulesWithGet;
use confluence::v1::shape::{ContentArray, User};
use confluence::v1::{
    CreateAttachment,
    DeleteSpace,
    DownloadAttatchment,
    GetCurrentUser,
    GetDescendantsOfType,
    SearchByCQL,
    get_current_user,
    search_by_cql,
};
use confluence::v2::shape::{
    BulkTransitionCombinationEntry,
    ClassificationLevel,
    GetPageByIDReply,
    GetSpaceByIDReply,
};
use confluence::v2::{
    CreatePage,
    GetClassificationLevels,
    GetDatabaseEntries,
    GetPageByID,
    GetSpaceByID,
    GetSpaces,
    ListSpacePermissionCombinations,
    get_database_entries,
    get_page_by_id,
    get_spaces,
    list_space_permission_combinations,
};
use confluence::{
    AccountID,
    Agent,
    Any,
    AttachmentID,
    Base,
    BlogPostID,
    Boolean,
    Client,
    CommentID,
    Credentials,
    Error,
    Field,
    Integer,
    List,
    Map,
    Method,
    Next,
    Number,
    Object,
    Operation,
    Page,
    PageID,
    Paged,
    Paging,
    Part,
    Request,
    Response,
    SpaceID,
    SpaceKey,
    Text,
    adf,
    limits,
};
use serde_json::{Value, json};

use crate::fake::{CLOUD_ID, Scripted, answer, client, dropped, response, url_of};

const FLAGS: Field<Shape, Map<Boolean>> = Field::new("flags");
const LABELS: Field<Shape, List<Text>> = Field::new("labels");
const PAGE: GetPageByID = GetPageByID { id: 1 };
const PARENT: Field<Shape, Object<Shape>> = Field::new("parent");
const RATIO: Field<Shape, Number> = Field::new("ratio");
const SIZE: Field<Shape, Integer> = Field::new("size");

const COMBINATION_ID: Field<BulkTransitionCombinationEntry, Text> =
    BulkTransitionCombinationEntry::COMBINATION_ID;

const DOWNLOAD: DownloadAttatchment<'static> =
    DownloadAttatchment { attachment_id: "att10", id: "1" };

#[derive(Debug)]
struct Custom<'call> {
    name: &'call str,
}

#[derive(Clone, Copy, Debug)]
struct Shape;

#[derive(Debug)]
struct Named<'call> {
    name: &'call str,
}

impl Operation for Named<'_> {
    const BASE: Base = Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: Method = Method::Get;
    const PATH: &'static str = "/wiki/rest/api/named/{name}";

    type Options<'options> = ();
    type Reply = Object<Shape>;

    fn request(&self, _options: &(), request: &mut Request) -> Result<(), Error> {
        assert!(!self.name.is_empty(), "a fixture names something");
        assert!(request.url().contains("{name}"), "the path holds its placeholder");

        request.path_text("name", self.name)
    }
}

impl Operation for Custom<'_> {
    const BASE: Base = Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: Method = Method::Get;
    const PATH: &'static str = "/wiki/rest/api/custom/{name}";

    type Options<'options> = ();
    type Reply = Any;

    fn request(&self, _options: &(), request: &mut Request) -> Result<(), Error> {
        assert!(!self.name.is_empty(), "a fixture names something");
        assert!(request.url().contains("{name}"), "the path holds its placeholder");

        request.path_text("name", self.name)
    }
}

#[test]
fn a_binary_operation_streams_into_its_sink_and_rewinds_on_a_retry() {
    let confluence = client(vec![dropped(), answer(200, &json!("bytes"))]);
    let mut sink = Cursor::new(b"stale bytes left by a dropped attempt".to_vec());
    let written = confluence.download(&DOWNLOAD, &mut sink).expect("a download");
    let seen = confluence.network().seen();

    assert_eq!(written, 7, "the seven bytes of the quoted word");
    assert_eq!(&sink.get_ref()[..7], b"\"bytes\"", "the retry wrote over the dropped attempt");
    assert_eq!(seen[0].accept, "*/*", "a download takes whatever Confluence sends");
    assert_eq!(seen.len(), 2, "a read repeats");
}

#[test]
fn a_comma_list_reaches_the_url_as_one_term_and_a_comma_inside_a_value_is_refused() {
    let confluence = client(vec![answer(200, &json!({"accountId": "5b10ac"}))]);
    let listed = get_current_user::Options { expand: Some(&["operations", "personalSpace"]) };
    let joined = get_current_user::Options { expand: Some(&["operations,personalSpace"]) };
    let _reply = confluence.send_with(&GetCurrentUser, &listed).expect("a user");
    let failure = confluence.send_with(&GetCurrentUser, &joined).expect_err("an ambiguous list");
    let seen = confluence.network().seen();

    let expected = url_of("/wiki/rest/api/user/current?expand=operations,personalSpace");

    assert_eq!(seen[0].url, expected, "one term, its values joined by literal commas");
    assert!(matches!(failure, Error::Input(_)), "the caller's fault: {failure:?}");
    assert_eq!(seen.len(), 1, "an ambiguous list is never sent");
}

#[test]
fn a_custom_operation_reaches_an_endpoint_no_specification_names() {
    let confluence = client(vec![answer(200, &json!({"ok": true}))]);
    let reply = confluence.send(&Custom { name: "a/b" }).expect("a reply");
    let seen = confluence.network().seen();

    assert_eq!(reply.value()["ok"], true, "the raw value reads");
    assert_eq!(seen[0].url, url_of("/wiki/rest/api/custom/a%2Fb"), "escaped");
}

#[test]
fn a_custom_shape_reads_every_kind_a_member_can_hold() {
    let body = json!({
        "flags": {"ready": true, "text": "no"},
        "labels": ["a", 7_i64, "b"],
        "parent": {"size": 3_i64},
        "ratio": 0.5_f64,
        "size": 9_i64,
    });

    let confluence = client(vec![answer(200, &body)]);
    let reply = confluence.send(&Named { name: "a" }).expect("a reply");
    let record = reply.read().expect("an object");
    let flags = record.member(FLAGS).expect("a map");
    let labels: Vec<&str> = record.member(LABELS).expect("a list").collect();
    let entries: Vec<(&str, bool)> = flags.entries().collect();
    let parent = record.member(PARENT).expect("a nested record");
    let one = json!(1_i64);
    let anything = <Any as confluence::Kind>::read(&one);

    assert_eq!(labels, ["a", "b"], "an item of another kind is passed over");
    assert_eq!(entries, [("ready", true)], "an entry of another kind is passed over");
    assert_eq!(parent.member(SIZE), Some(3), "a record nests in its own schema");
    assert_eq!(anything, Some(&one), "anything at all");
    assert_eq!(flags.member("ready"), Some(true), "a map member reads by key");
    assert_eq!(flags.count(), 2, "both members count");
    assert_eq!(record.member(RATIO), Some(0.5_f64), "a number");
}

#[test]
fn a_download_at_its_guard_lands_and_one_byte_past_it_is_refused() {
    let mut exact = response(200, &Value::Null);
    let mut over = response(200, &Value::Null);

    exact.streamed_count = limits::ATTACHMENT_BYTES_MAX;
    over.streamed_count = limits::ATTACHMENT_BYTES_MAX.strict_add(1);

    let confluence = client(vec![Scripted::Answered(exact), Scripted::Answered(over)]);
    let written = confluence.download(&DOWNLOAD, &mut Cursor::new(Vec::new())).expect("at it");
    let failure = confluence.download(&DOWNLOAD, &mut Cursor::new(Vec::new())).expect_err("past");

    assert_eq!(written, limits::ATTACHMENT_BYTES_MAX, "the guard itself is allowed");
    assert!(matches!(failure, Error::Reply(_)), "one byte past the guard is refused");
}

#[test]
fn a_download_keeps_the_position_the_sink_was_handed_at() {
    let confluence = client(vec![dropped(), answer(200, &json!("new"))]);
    let mut sink = Cursor::new(b"header:".to_vec());

    sink.set_position(7);

    let written = confluence.download(&DOWNLOAD, &mut sink).expect("a download");

    assert_eq!(written, 5, "the five bytes of the quoted word");
    assert_eq!(sink.get_ref().as_slice(), b"header:\"new\"", "what came before stays");
}

#[test]
fn a_failed_status_is_a_refusal_that_keeps_the_status() {
    let refused =
        json!({"errors": [{"code": "NOT_FOUND", "status": 404_i64, "title": "Not Found"}]});

    let confluence = client(vec![answer(404, &refused)]);
    let failure = confluence.send(&PAGE).expect_err("a refusal");

    assert_eq!(failure.status(), Some(404), "the status");
    assert!(failure.message().contains("NOT_FOUND"), "the body is excerpted");
    assert!(failure.to_string().contains("/wiki/api/v2/pages/1"), "the request is named");
}

#[test]
fn a_header_option_reaches_the_request_and_a_deep_object_its_query() {
    let confluence = client(vec![answer(200, &json!({"results": []}))]);
    let filter = [("status", "done")];

    let options = get_database_entries::Options {
        cursor: None,
        filter: Some(&filter),
        if_none_match: Some("\"tag\""),
        limit: None,
        sort: None,
    };

    let _reply = confluence.send_with(&GetDatabaseEntries { id: "7" }, &options).expect("read");
    let seen = confluence.network().seen();
    let header = ("If-None-Match".to_owned(), "\"tag\"".to_owned());
    let expected = url_of("/wiki/api/v2/databases/7/entries?filter%5Bstatus%5D=done");

    assert!(seen[0].headers.contains(&header), "the entity tag is sent");
    assert_eq!(seen[0].url, expected, "the key travels inside its brackets, escaped");
}

#[test]
fn a_landed_write_with_an_unreadable_body_is_still_a_success() {
    let mut landed = response(201, &Value::Null);

    landed.body = b"<html>created</html>".to_vec();

    let confluence = client(vec![Scripted::Answered(landed)]);
    let body = json!({"spaceId": "1", "title": "words"});
    let reply = confluence.send(&CreatePage { body: &body }).expect("the write landed");

    assert_eq!(reply.value(), &Value::Null, "the body could not be read");
    assert_eq!(confluence.network().seen().len(), 1, "and nothing was repeated");
}

#[test]
fn a_link_walk_reads_the_next_cursor_out_of_the_address_confluence_returns() {
    let first = json!({
        "data": [{"id": "1"}],
        "links": {"next": "https://api.atlassian.com/rules?limit=1&cursor=c%2F2"},
    });

    let second = json!({"data": [{"id": "2"}], "links": {"next": null}});
    let confluence = client(vec![answer(200, &first), answer(200, &second)]);
    let operation = SearchManualRulesWithGet { cursor: "start" };
    let pages: Vec<_> = confluence.walk(&operation).collect();
    let seen = confluence.network().seen();

    assert_eq!(pages.len(), 2, "both pages were walked");
    assert!(seen[0].url.ends_with("cursor=start"), "the first page uses what the caller gave");
    assert!(seen[1].url.ends_with("cursor=c%2F2"), "the second replaces it, escaped once");
    assert_eq!(seen[1].url.matches("cursor=").count(), 1, "the term is replaced, not repeated");
}

#[test]
fn a_list_reply_is_read_by_hand_rather_than_walked() {
    let body = json!([{"id": "1", "name": "Public"}, 7_i64, {"id": "2", "name": "Secret"}]);
    let confluence = client(vec![answer(200, &body)]);
    let reply = confluence.send(&GetClassificationLevels).expect("one window");
    let levels = reply.read().expect("a list");
    let names = levels.filter_map(|level| level.member(ClassificationLevel::NAME));

    assert_eq!(names.collect::<Vec<_>>(), ["Public", "Secret"], "each level in order");
    assert_eq!(confluence.network().seen().len(), 1, "one window");
}

#[test]
fn a_multipart_body_reaches_the_wire_with_its_xsrf_header() {
    let uploaded = json!({"results": [{"id": "att10"}], "size": 1_i64});
    let confluence = client(vec![answer(200, &uploaded)]);

    let parts = [Part {
        bytes: b"hello",
        content_type: "text/plain",
        file_name: Some("a.txt"),
        name: "file",
    }];

    let reply = confluence.send(&CreateAttachment { body: &parts, id: "123" }).expect("sent");
    let seen = confluence.network().seen();
    let body = String::from_utf8(seen[0].body.clone()).expect("plain text");
    let attached = reply.read().and_then(|array| array.member(ContentArray::RESULTS));

    assert_eq!(seen[0].method, Method::Post, "the method");
    assert!(body.contains("filename=\"a.txt\""), "the part is named");
    assert!(body.contains("hello"), "the part is sent");
    assert!(seen[0].headers.contains(&header_xsrf()), "the header the endpoint demands");
    assert_eq!(attached.map_or(0, Iterator::count), 1, "the reply lists the attachment");
}

#[test]
fn a_near_limit_answer_pauses_after_it_succeeds() {
    let mut paced = response(200, &json!({"id": "1"}));

    paced.is_near_limit = true;

    let confluence = client(vec![Scripted::Answered(paced)]);
    let _reply = confluence.send(&PAGE).expect("a reply");
    let waits = confluence.network().waits();

    assert_eq!(waits.len(), 1, "one pause");
    assert_eq!(waits[0].reason, "near the limit", "the pause says why");
    assert_eq!(waits[0].millis, 250, "the documented pause");
    assert!(waits[0].note.is_none(), "a pause is no failure");
}

#[test]
fn a_part_is_plain_data_a_caller_fills_in() {
    let part = Part { bytes: b"\x89PNG", content_type: "image/png", file_name: None, name: "file" };

    assert_eq!(part.content_type, "image/png", "the type");
    assert_eq!(part.bytes.len(), 4, "the bytes");
}

#[test]
fn a_pause_that_fails_cannot_lose_a_landed_write() {
    let mut paced = response(200, &json!({"id": "2"}));

    paced.is_near_limit = true;

    let confluence = client(vec![Scripted::Answered(paced)]);

    confluence.network().wait_refuse();

    let body = json!({"spaceId": "1", "title": "words"});
    let reply = confluence.send(&CreatePage { body: &body }).expect("the write landed");

    assert_eq!(reply.value()["id"], "2", "the reply survives the failed pause");
    assert_eq!(confluence.network().waits().len(), 1, "the pause was attempted");
}

#[test]
fn a_read_retries_a_server_failure_and_says_why_it_waited() {
    let confluence = client(vec![answer(503, &Value::Null), answer(200, &json!({"title": "A"}))]);
    let reply = confluence.send(&PAGE).expect("the second attempt");
    let waits = confluence.network().waits();
    let title = reply.read().and_then(|page| page.member(GetPageByIDReply::TITLE));

    assert_eq!(title, Some("A"), "the second answer is read");
    assert_eq!(waits.len(), 1, "one retry");
    assert_eq!(waits[0].reason, "server error", "the wait says why");
    assert!(waits[0].millis >= 3_500, "the first delay is jittered around five seconds");
    assert!(waits[0].note.as_deref().is_some_and(|note| note.contains("retry 1 of 4")), "noted");
}

#[test]
fn a_read_that_keeps_failing_gives_up_after_four_retries_and_names_the_request() {
    let attempts = vec![dropped(), dropped(), dropped(), dropped(), dropped()];
    let confluence = client(attempts);
    let failure = confluence.send(&PAGE).expect_err("a lost network");

    assert!(matches!(failure, Error::Transport(_)), "a read is safe to call a transport failure");
    assert!(failure.message().contains("/wiki/api/v2/pages/1"), "the request is named");
    assert_eq!(confluence.network().seen().len(), 5, "the first attempt and four retries");
    assert_eq!(confluence.network().answers_left(), 0, "the script ran out exactly");
}

#[test]
fn a_record_reads_typed_members_and_keeps_the_rest_of_the_value() {
    let body = json!({
        "customThing": 7_i64,
        "homepageId": "65538",
        "key": "DEV",
        "name": "Development",
    });

    let confluence = client(vec![answer(200, &body)]);
    let reply = confluence.send(&GetSpaceByID { id: 98_306 }).expect("a space");
    let space = reply.read().expect("an object");

    assert_eq!(space.member(GetSpaceByIDReply::KEY), Some("DEV"), "a text");
    assert_eq!(space.member(GetSpaceByIDReply::HOMEPAGE_ID), Some("65538"), "an id as text");
    assert_eq!(space.member(GetSpaceByIDReply::ID), None, "a member the reply lacks is absent");
    assert_eq!(space.value()["customThing"], 7_i64, "what the schema never named stays");
}

#[test]
fn a_redirect_on_a_write_is_the_task_it_started() {
    let task = "https://site/wiki/rest/api/longtask/10";
    let mut accepted = response(303, &Value::Null);

    accepted.location = Some(task.to_owned());

    let confluence = client(vec![Scripted::Answered(accepted)]);
    let reply = confluence.send(&DeleteSpace { space_key: "DEV" }).expect("an accepted task");

    assert_eq!(reply.location(), Some(task), "the task to poll");
    assert_eq!(confluence.network().seen()[0].method, Method::Delete, "the method");
}

#[test]
fn a_repeated_token_stops_the_walk_after_the_page_it_arrived_with() {
    let first = json!({"cursor": "t", "results": [{"combinationId": "1"}]});
    let second = json!({"cursor": "t", "results": [{"combinationId": "2"}]});
    let confluence = client(vec![answer(200, &first), answer(200, &second)]);
    let pages: Vec<_> = confluence.walk(&ListSpacePermissionCombinations).collect();

    assert_eq!(pages.len(), 3, "two pages, then the refusal");
    assert!(pages[1].is_ok(), "the second page still arrives");
    assert!(matches!(pages[2], Err(Error::Reply(_))), "and the loop is named rather than run");
}

#[test]
fn a_required_parameter_is_a_field_and_an_option_is_a_second_literal() {
    let confluence = client(vec![answer(200, &json!({"id": "1"})), answer(200, &json!({}))]);

    let options = get_page_by_id::Options {
        body_format: Some("storage"),
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
        status: Some(&["current", "draft"]),
        version: None,
    };

    let _bare = confluence.send(&PAGE).expect("a reply");
    let _configured = confluence.send_with(&PAGE, &options).expect("a reply");
    let seen = confluence.network().seen();

    assert_eq!(seen[0].url, url_of("/wiki/api/v2/pages/1"), "no option, no query");
    assert!(seen[0].authorization.starts_with("Basic "), "every request carries the credential");

    assert_eq!(
        seen[1].url,
        url_of("/wiki/api/v2/pages/1?body-format=storage&status=current&status=draft"),
        "each option under its wire name",
    );
}

#[test]
fn a_retry_after_at_its_guard_is_honored_and_one_second_more_ends_the_retries() {
    let mut honored = response(429, &Value::Null);
    let mut refused = response(429, &Value::Null);

    honored.retry_after_seconds = Some(900);
    refused.retry_after_seconds = Some(901);

    let confluence = client(vec![Scripted::Answered(honored), answer(200, &json!({}))]);
    let cut_short = client(vec![Scripted::Answered(refused)]);

    assert!(confluence.send(&GetCurrentUser).is_ok(), "a wait of 15 minutes is honored");
    assert!(confluence.network().waits()[0].millis >= 900_000, "the wait Confluence asked for");

    let failure = cut_short.send(&GetCurrentUser).expect_err("a wait past the guard ends it");

    assert_eq!(failure.status(), Some(429), "the refusal comes back");
    assert!(cut_short.network().waits().is_empty(), "nothing was waited for");
}

#[test]
fn a_retry_after_is_a_floor_the_jitter_never_undercuts() {
    for _attempt_index in 0..32_u32 {
        let mut throttled = response(429, &Value::Null);

        throttled.retry_after_seconds = Some(30);

        let confluence = client(vec![Scripted::Answered(throttled), answer(200, &json!({}))]);

        assert!(confluence.send(&GetCurrentUser).is_ok(), "the second attempt lands");
        assert!(confluence.network().waits()[0].millis >= 30_000, "the jitter stays above it");
    }
}

#[test]
fn a_rewritten_authorization_never_reaches_a_diagnostic() {
    let credentials =
        Credentials::Token { cloud_id: CLOUD_ID, email: "user@example.com", token: "s3cret" };

    let confluence = Client::new(Agent::new(), &credentials).expect("a client");

    assert!(!format!("{credentials:?}").contains("s3cret"), "the credentials hide the token");
    assert!(!format!("{confluence:?}").contains("s3cret"), "the client hides the token");
    assert!(format!("{confluence:?}").contains(CLOUD_ID), "and names the site");
}

#[test]
fn a_search_walk_replaces_its_cursor_and_keeps_the_query_it_was_given() {
    let next = "/rest/api/search?next=true&cursor=c2&limit=1&start=1&cql=type%3Dpage";
    let first = json!({"_links": {"next": next}, "results": [{"title": "A"}], "totalSize": 2_i64});
    let second = json!({"_links": {}, "results": [{"title": "B"}], "totalSize": 2_i64});
    let confluence = client(vec![answer(200, &first), answer(200, &second)]);

    let options = search_by_cql::Options {
        cache_buster: None,
        cqlcontext: None,
        cursor: None,
        excerpt: None,
        exclude_spaces_current: None,
        expand: None,
        include_archived_spaces: None,
        limit: Some(1),
        next: Some(true),
        previous: None,
        site_permission_type_filter: None,
        start: None,
    };

    let operation = SearchByCQL { cql: "type = page" };
    let pages: Vec<_> = confluence.walk_with(&operation, options).collect();
    let totals: Vec<Option<u64>> = pages.iter().flatten().map(Page::count_total).collect();
    let seen = confluence.network().seen();

    assert_eq!(totals, [Some(2), Some(2)], "both pages, each with the total it reported");
    assert!(!seen[0].url.contains("cursor="), "the first page names no cursor");
    assert!(seen[1].url.contains("cql=type%20%3D%20page"), "the query rides on every page");
    assert!(seen[1].url.contains("next=true"), "and so does every option: {}", seen[1].url);
    assert!(seen[1].url.ends_with("cursor=c2"), "the cursor is the one the link named");
}

#[test]
fn a_start_walk_moves_by_the_start_its_next_link_names() {
    let next = "/rest/api/content/1/descendant/page?limit=2&start=2";
    let first = json!({"_links": {"next": next}, "results": [{"id": "2"}, {"id": "3"}]});
    let second = json!({"_links": {"base": "https://x/wiki"}, "results": [{"id": "4"}]});
    let confluence = client(vec![answer(200, &first), answer(200, &second)]);
    let operation = GetDescendantsOfType { id: "1", kind: "page" };
    let mut ids: Vec<String> = Vec::new();

    for walked in confluence.walk(&operation) {
        let values = walked.expect("a page").into_values();

        ids.extend(values.iter().filter_map(|item| item["id"].as_str()).map(str::to_owned));
    }

    let seen = confluence.network().seen();

    assert_eq!(ids, ["2", "3", "4"], "every descendant in order");
    assert!(!seen[0].url.contains("start="), "the first page names no start");
    assert!(seen[1].url.ends_with("start=2"), "the second starts where the link said");
}

#[test]
fn a_throttled_write_is_retried_because_it_never_landed() {
    let mut throttled = response(429, &Value::Null);

    throttled.retry_after_seconds = Some(1);
    throttled.rate_limit_reason = Some("confluence-quota-tenant-based".to_owned());

    let confluence = client(vec![Scripted::Answered(throttled), answer(200, &json!({"id": "2"}))]);
    let body = json!({"spaceId": "1", "title": "words"});
    let reply = confluence.send(&CreatePage { body: &body }).expect("the second attempt");
    let waits = confluence.network().waits();
    let note = waits[0].note.as_deref().unwrap_or_default();

    assert_eq!(reply.value()["id"], "2", "created");
    assert_eq!(waits[0].reason, "rate limited", "the wait says why");
    assert!(waits[0].millis >= 1_000, "the delay Confluence asked for is a floor");
    assert!(note.contains("confluence-quota-tenant-based"), "the reason given is named: {note}");
}

#[test]
fn a_token_walk_follows_its_tokens_to_the_last_page() {
    let first = json!({"cursor": "t2", "results": [{"combinationId": "1"}]});
    let second = json!({"results": [{"combinationId": "2"}]});
    let confluence = client(vec![answer(200, &first), answer(200, &second)]);
    let options = list_space_permission_combinations::Options { cursor: None, limit: Some(1) };
    let mut ids: Vec<String> = Vec::new();

    for walked in confluence.walk_with(&ListSpacePermissionCombinations, options) {
        let found = walked.expect("a page");
        let entries = found.items().filter_map(|entry| entry.member(COMBINATION_ID));

        ids.extend(entries.map(str::to_owned));
    }

    let seen = confluence.network().seen();

    assert_eq!(ids, ["1", "2"], "both pages in order");
    assert!(!seen[0].url.contains("cursor"), "the first page names no token");
    assert!(seen[1].url.ends_with("cursor=t2"), "the second names the token");
    assert!(seen[1].url.contains("limit=1"), "the options ride on every page");
}

#[test]
fn a_walk_prints_where_it_stands() {
    let confluence = client(vec![answer(200, &json!({"results": []}))]);
    let walk = confluence.walk(&GetSpaces);
    let printed = format!("{walk:?}");

    assert!(printed.starts_with("Walk"), "the type is named: {printed}");
    assert!(printed.contains("page_count_remaining"), "the budget shows: {printed}");
    assert!(!printed.contains("secret"), "no credential prints: {printed}");
}

#[test]
fn a_walk_starts_where_the_options_say_and_moves_forward_from_there() {
    let next = "/wiki/api/v2/spaces?cursor=c2";
    let first = json!({"_links": {"next": next}, "results": [{"key": "A"}, {"key": "B"}]});
    let second = json!({"_links": {}, "results": [{"key": "C"}]});
    let confluence = client(vec![answer(200, &first), answer(200, &second)]);

    let options = get_spaces::Options {
        cursor: Some("c1"),
        description_format: None,
        favorited_by: None,
        ids: None,
        include_icon: None,
        keys: None,
        kind: None,
        labels: None,
        limit: None,
        not_favorited_by: None,
        sort: None,
        status: None,
    };

    let pages: Vec<_> = confluence.walk_with(&GetSpaces, options).collect();
    let seen = confluence.network().seen();

    assert_eq!(pages.len(), 2, "both pages");
    assert!(seen[0].url.ends_with("cursor=c1"), "the first page starts where it was told");
    assert!(seen[1].url.ends_with("cursor=c2"), "the second follows its link");
}

#[test]
fn a_walk_stops_at_the_first_failure() {
    let first = json!({"_links": {"next": "/wiki/api/v2/spaces?cursor=c2"}, "results": [{}]});
    let confluence = client(vec![answer(200, &first), answer(404, &Value::Null)]);
    let pages: Vec<_> = confluence.walk(&GetSpaces).collect();

    assert_eq!(pages.len(), 2, "the page, then the failure, then nothing");
    assert!(pages[0].is_ok(), "the first page arrived");
    assert_eq!(pages[1].as_ref().err().and_then(Error::status), Some(404), "the refusal");
}

#[test]
fn a_write_is_not_repeated_after_a_failure_that_may_have_landed() {
    let lost = client(vec![dropped()]);
    let failed = client(vec![answer(503, &Value::Null)]);
    let body = json!({"spaceId": "1", "title": "words"});
    let operation = CreatePage { body: &body };
    let indefinite = lost.send(&operation).expect_err("a lost write");
    let refused = failed.send(&operation).expect_err("a failed write");

    assert!(matches!(indefinite, Error::Indefinite(_)), "the write may have landed");
    assert!(indefinite.message().contains("may have landed"), "and the message says so");
    assert!(matches!(refused, Error::Indefinite(_)), "a gateway may have let the write through");
    assert!(refused.message().contains("503"), "and the status stays in the message");
    assert_eq!(lost.network().seen().len(), 1, "one attempt");
    assert_eq!(failed.network().seen().len(), 1, "one attempt");
}

#[test]
fn a_write_sends_its_json_and_reads_its_reply() {
    let confluence = client(vec![answer(200, &json!({"id": "10", "title": "words"}))]);
    let body = json!({"spaceId": "1", "status": "current", "title": "words"});
    let reply = confluence.send(&CreatePage { body: &body }).expect("created");
    let seen = confluence.network().seen();

    assert_eq!(seen[0].method, Method::Post, "the method");
    assert_eq!(seen[0].content_type.as_deref(), Some("application/json"), "the type");
    assert_eq!(seen[0].body, serde_json::to_vec(&body).expect("JSON"), "the bytes");
    assert!(!seen[0].is_read, "a create is a write");
    assert_eq!(reply.value()["id"], "10", "the reply");
}

#[test]
fn an_empty_page_with_a_token_keeps_the_walk_going() {
    let filtered = json!({"cursor": "t2", "results": []});
    let last = json!({"results": [{"combinationId": "1"}]});
    let confluence = client(vec![answer(200, &filtered), answer(200, &last)]);
    let pages: Vec<_> = confluence.walk(&ListSpacePermissionCombinations).collect();

    assert_eq!(pages.len(), 2, "the empty page does not end the walk");
    assert_eq!(pages[1].as_ref().map_or(0, Page::count), 1, "the last page arrives");
}

#[test]
fn credentials_that_could_not_reach_a_site_are_refused_at_construction() {
    let hostile = Credentials::Bearer { cloud_id: "tenant/../other", token: "x" };
    let empty = Credentials::Bearer { cloud_id: "site", token: "" };

    assert!(matches!(Client::new(Agent::new(), &hostile), Err(Error::Input(_))), "a bad site");
    assert!(matches!(Client::new(Agent::new(), &empty), Err(Error::Input(_))), "no token");
}

#[test]
fn each_product_reaches_its_own_base() {
    let empty = json!({});
    let rules = json!({"data": []});
    let confluence = client(vec![answer(200, &rules), answer(200, &empty), answer(200, &empty)]);
    let _rules = confluence.send(&SearchManualRulesWithGet { cursor: "c" }).expect("a reply");
    let _user = confluence.send(&GetCurrentUser).expect("a reply");
    let _spaces = confluence.send(&GetSpaces).expect("a reply");
    let seen = confluence.network().seen();
    let base = format!("https://api.atlassian.com/automation/public/confluence/{CLOUD_ID}");

    assert_eq!(seen[0].url, format!("{base}/rest/v1/rule/manual/search?cursor=c"), "automation");
    assert_eq!(seen[1].url, url_of("/wiki/rest/api/user/current"), "the v1 API");
    assert_eq!(seen[2].url, url_of("/wiki/api/v2/spaces"), "the v2 API");
}

fn header_xsrf() -> (String, String) {
    let header = ("X-Atlassian-Token".to_owned(), "no-check".to_owned());

    assert!(!header.0.is_empty(), "the header is named");
    assert!(!header.1.is_empty(), "the header has a value");

    header
}

#[test]
fn the_paged_trait_names_how_each_operation_walks() {
    let spaces = <GetSpaces as Paged>::PAGING;
    let combinations = <ListSpacePermissionCombinations as Paged>::PAGING;
    let rules = <SearchManualRulesWithGet<'_> as Paged>::PAGING;
    let descendants = <GetDescendantsOfType<'_> as Paged>::PAGING;

    let linked = Paging { cursor: "cursor", items: "/results", next: Next::Link("/_links/next") };

    assert_eq!(spaces, linked, "a cursor the next link names");
    assert_eq!(combinations.next, Next::Token("/cursor"), "a bare token");
    assert_eq!(rules.items, "/data", "the automation API");
    assert_eq!(rules.next, Next::Link("/links/next"), "the automation API");
    assert_eq!(descendants.cursor, "start", "an offset the link names");
}

#[test]
fn the_response_a_custom_transport_builds_is_plain_data() {
    let built = Response {
        body: b"{}".to_vec(),
        is_near_limit: false,
        location: None,
        rate_limit_reason: None,
        retry_after_seconds: Some(5),
        status: 200,
        streamed_count: 0,
    };

    let confluence = client(vec![Scripted::Answered(built)]);

    assert_eq!(confluence.send(&GetSpaces).expect("a reply").value(), &json!({}), "the body");
    assert_eq!(confluence.network().waits().len(), 0, "a success never waits");
}

#[test]
fn the_rich_text_flattener_reads_a_document_as_prose() {
    let document = json!({
        "type": "doc",
        "content": [{"type": "paragraph", "content": [{"type": "text", "text": "hello"}]}],
    });

    assert_eq!(adf::text_read(&document), "hello", "a document");
    assert_eq!(adf::text_read(&json!("plain")), "plain", "a plain string passes through");
    assert!(adf::TRUNCATION_NOTE.contains("truncated"), "the marker names itself");
}

#[test]
fn the_typed_handles_are_valid_by_construction() {
    assert_eq!(SpaceKey::parse("DEV").expect("a key").text(), "DEV", "a space key");
    assert!(SpaceKey::parse("~5b10ac").expect("a key").is_personal(), "a personal space");
    assert_eq!(AccountID::parse("5b10ac").expect("an id").text(), "5b10ac", "an account");
    assert_eq!(AttachmentID::parse("att7").expect("an id").text(), "att7", "an attachment");
    assert!(SpaceKey::parse("a\"b").is_none(), "a quote closes a CQL term");
    assert!(AccountID::parse("a b").is_none(), "a name is not an account id");
    assert!(AttachmentID::parse("att").is_none(), "a prefix alone is no id");
    assert!(PageID::new(0).is_none(), "zero is no id");
    assert_eq!(BlogPostID::new(7).expect("an id").get(), 7, "a blog post");
    assert_eq!(CommentID::new(7).expect("an id").get(), 7, "a comment");
    assert_eq!(PageID::new(7).expect("an id").get(), 7, "a page");
    assert_eq!(SpaceID::new(7).expect("an id").get(), 7, "a space");
}

#[test]
fn the_url_limit_is_a_refusal_rather_than_a_request_confluence_would_reject() {
    let confluence = client(vec![answer(200, &json!({}))]);
    let long = "x".repeat(usize::try_from(limits::URL_BYTES_MAX).expect("a small limit"));
    let failure = confluence.send(&DeleteSpace { space_key: &long }).expect_err("too long");

    assert!(matches!(failure, Error::Input(_)), "the caller's fault");
    assert!(confluence.network().seen().is_empty(), "nothing was sent");
}

#[test]
fn the_user_shape_reads_through_a_list() {
    let body = json!([{"displayName": "Ada"}, 7_i64, {"displayName": "Bo"}]);
    let items = body.as_array().expect("an array");
    let names = items.iter().filter_map(|value| value.get("displayName"));

    assert_eq!(names.count(), 2, "the fixture holds two users");
    assert_eq!(User::DISPLAY_NAME.name(), "displayName", "the wire name");
}
