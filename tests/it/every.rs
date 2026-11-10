use alloc::collections::BTreeMap;
use core::iter;

use confluence::Response;
use serde_json::{Map, Value, json};

use crate::conformance::document_read;
use crate::fake::{self, CLOUD_ID, FakeNetwork, Scripted, Seen};
use crate::step::{Run, Sent, Step, WIRE_BODY, WIRE_PARTS};
use crate::sweep::{automation, v1, v2};

const ACCEPT_ANY: &str = "*/*";
const ACCEPT_JSON: &str = "application/json";
const ALL_OF_COUNT_MAX: usize = 8;
const BODY_BYTES_MAX: usize = 1_000_000;
const CONTENT_JSON: &str = "application/json";
const CONTENT_MULTIPART: &str = "multipart/form-data; boundary=";
const CURSOR_LINKED: &str = "next%2Fpage%20two";
const CURSOR_NEXT: &str = "next/page two";
const DOWNLOAD_BYTES: &[u8] = b"PK\x03\x04 sweep \xff";
const FAILURE_COUNT_MAX: usize = 64;
const FAILURE_SHOWN_COUNT_MAX: u8 = 40;
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
const GATEWAY: &str = "https://api.atlassian.com";
const HEADER_COUNT_MAX: usize = 16;
const HEADER_XSRF: (&str, &str) = ("X-Atlassian-Token", "no-check");
const INTEGERS_READ_AS_TEXT: [(&str, &str); 1] = [("getSpaceRolesById", "id")];
const LIST_SEPARATOR: &str = "|";
const LOCATION_ANSWERED: &str = "https://api.atlassian.com/ex/confluence/task/7";
const METHODS: [&str; 5] = ["delete", "get", "patch", "post", "put"];
const OPERATION_COUNT_TOTAL: u64 = 397;
const PAGE_COUNT_WALKED: u64 = 2;
const PAGED_COUNT_TOTAL: u64 = 107;
const QUERY_TERM_COUNT_MAX: usize = 128;

const READS_BY_POST: [&str; 7] = [
    "checkAccessByEmail",
    "checkContentPermission",
    "convertContentIdsToContentTypes",
    "createBulkUserLookup",
    "searchManualRulesWithPost",
    "searchRuleSummaries",
    "searchTemplatesWithPost",
];

const REFERENCE_HOP_COUNT_MAX: u32 = 16;
const START_NEXT: &str = "26";
const TEXT_SALT: &str = " /?#&=+%[]\u{e9}";
const URL_BYTES_MAX: usize = 8_192;
const VALUE_COUNT_MAX: usize = 32;

#[derive(Clone, Copy, Debug)]
struct Body {
    is_multipart: bool,
    is_required: bool,
}

#[derive(Clone, Copy, Debug)]
struct Declared<'spec> {
    is_required: bool,
    place: Place,
    shape: Shape,
    wire: &'spec str,
}

#[derive(Clone, Copy, Debug)]
struct Document<'spec> {
    root: &'spec Value,
}

#[derive(Clone, Copy, Debug)]
struct Located<'spec> {
    item: &'spec Value,
    method: &'spec str,
    operation: &'spec Value,
    path: &'spec str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Full,
    Required,
    Walk,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Next {
    Link(&'static str),
    Token,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Place {
    Header,
    Path,
    Query,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Reply {
    Binary,
    JSONValue,
    Nothing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Shape {
    Boolean,
    Decimal,
    Integer,
    Integers,
    Pairs,
    Text,
    Texts,
    TextsJoined,
}

#[derive(Debug)]
struct Specified<'spec> {
    body: Option<Body>,
    identifier: &'spec str,
    is_xsrf: bool,
    method: &'spec str,
    parameters: Vec<Declared<'spec>>,
    path: String,
    reply: Reply,
    reply_status: u16,
    reply_value: Value,
    walked: Option<Walked>,
}

#[derive(Clone, Copy, Debug)]
struct Swept<'sweep> {
    run: Run<FakeNetwork>,
    specified: &'sweep Specified<'sweep>,
}

type Table<'table> = &'table [(&'table str, Run<FakeNetwork>)];

#[derive(Debug)]
struct Walked {
    cursor: &'static str,
    item: Value,
    items: &'static str,
    next: Next,
}

impl<'spec> Document<'spec> {
    fn base(self) -> String {
        let server = self.root.pointer("/servers/0/url").and_then(Value::as_str).expect("a server");
        let address = server.trim_start_matches("https:").trim_start_matches("//");
        let (host, path) = address.split_once('/').unwrap_or((address, ""));
        let trimmed = path.trim_end_matches('/');
        let tail = if trimmed.is_empty() { String::new() } else { format!("/{trimmed}") };

        assert!(!host.is_empty(), "a server names its host");

        if host != "api.atlassian.com" {
            return format!("{GATEWAY}/ex/confluence/{CLOUD_ID}{tail}");
        }

        let templated = format!("{GATEWAY}{tail}").replace("{product}", "confluence");

        assert!(templated.contains("{cloudid}"), "a gateway server names its site by cloud id");

        templated.replace("{cloudid}", CLOUD_ID)
    }

    fn body_of(self, raw: &'spec Value) -> Body {
        let body = self.resolved(raw);
        let content = body.get("content").and_then(Value::as_object).expect("a body's media");
        let is_multipart = content.contains_key("multipart/form-data");
        let is_json = content.keys().any(|media| media_is_json(media));
        let is_required = body.get("required").and_then(Value::as_bool).unwrap_or(false);

        assert!(!content.is_empty(), "a body names at least one media type");
        assert_ne!(is_multipart, is_json, "a body is either JSON or multipart");

        Body { is_multipart, is_required }
    }

    fn declared_of(self, parameter: &'spec Value) -> Declared<'spec> {
        assert!(parameter.is_object(), "a parameter is an object");

        let wire = parameter.get("name").and_then(Value::as_str).expect("a named parameter");
        let is_required = parameter.get("required").and_then(Value::as_bool).unwrap_or(false);
        let shape = self.shape_of(parameter);

        let place = match parameter.get("in").and_then(Value::as_str) {
            Some("header") => Place::Header,
            Some("path") => Place::Path,
            Some("query") => Place::Query,
            other => panic!("{wire} sits in {other:?}, where the sweep sends nothing"),
        };

        if place == Place::Path {
            assert!(is_required, "the path parameter {wire} is required");
        }

        if place == Place::Header {
            assert_eq!(shape, Shape::Text, "the header {wire} is text");
        }

        Declared { is_required, place, shape, wire }
    }

    fn members_of(self, schema: &'spec Value) -> BTreeMap<&'spec str, &'spec Value> {
        let all_of = schema.get("allOf").and_then(Value::as_array).map_or(&[][..], Vec::as_slice);
        let mut members: BTreeMap<&'spec str, &'spec Value> = BTreeMap::new();

        assert!(all_of.len() <= ALL_OF_COUNT_MAX, "a schema merges a few others");

        let merged = all_of.iter().map(|raw| self.resolved(raw));

        for part in iter::once(schema).chain(merged) {
            let properties = part.get("properties").and_then(Value::as_object);

            for (name, raw) in properties.into_iter().flatten() {
                let _replaced = members.insert(name.as_str(), self.resolved(raw));
            }
        }

        assert!(members.values().all(|found| found.get("$ref").is_none()), "each resolves");

        members
    }

    fn resolved(self, node: &'spec Value) -> &'spec Value {
        assert!(matches!(node, Value::Object(_) | Value::Null), "a node is an object");

        let mut current = node;

        for _hop_index in 0..REFERENCE_HOP_COUNT_MAX {
            let Some(reference) = current.get("$ref").and_then(Value::as_str) else {
                return current;
            };

            let pointer = reference.strip_prefix('#').expect("a reference inside its document");

            current = self.root.pointer(pointer).expect("a reference names a node");
        }

        panic!("a reference chain ran past {REFERENCE_HOP_COUNT_MAX} hops");
    }

    fn sample_of(self, schema: &'spec Value) -> Value {
        assert!(matches!(schema, Value::Object(_) | Value::Null), "a schema is an object");

        let found = self.resolved(schema);

        let sample = match found.get("type").and_then(Value::as_str) {
            Some("array") => json!([]),
            Some("boolean") => json!(true),
            Some("integer" | "number") => json!(7_i64),
            Some("string") => json!("sentinel"),
            Some("object") | None => json!({"sentinel": true}),
            Some(other) => panic!("a reply of type {other} reads as no kind the sweep knows"),
        };

        assert!(!sample.is_null(), "a sample holds a value");

        sample
    }

    fn schema_of(self, response: &'spec Value) -> Option<&'spec Value> {
        assert!(response.is_object(), "a response is an object");

        let content = response.get("content").and_then(Value::as_object)?;
        let (_media, found) = content.iter().find(|(media, _found)| media_is_json(media))?;
        let schema = self.resolved(found.get("schema").unwrap_or(&Value::Null));

        assert!(schema.get("$ref").is_none(), "a schema resolves");

        Some(schema)
    }

    fn shape_of(self, parameter: &'spec Value) -> Shape {
        assert!(parameter.get("name").is_some(), "a parameter is named");

        let schema = self.resolved(parameter.get("schema").unwrap_or(&Value::Null));
        let style = parameter.get("style").and_then(Value::as_str);
        let is_exploded = parameter.get("explode").and_then(Value::as_bool).unwrap_or(true);
        let items = schema.get("items").map(|found| self.resolved(found));
        let item_type = items.and_then(|found| found.get("type")).and_then(Value::as_str);

        let shape = match (schema.get("type").and_then(Value::as_str), style) {
            (Some("object"), Some("deepObject")) => Shape::Pairs,
            (Some("array"), _) if item_type == Some("integer") => Shape::Integers,
            (Some("array"), _) if is_exploded => Shape::Texts,
            (Some("array"), _) => Shape::TextsJoined,
            (Some("boolean"), _) => Shape::Boolean,
            (Some("integer"), _) => Shape::Integer,
            (Some("number"), _) => Shape::Decimal,
            (Some("string") | None, _) => Shape::Text,
            (Some(other), _) => panic!("a parameter of type {other} has no shape the sweep sends"),
        };

        if shape == Shape::Integers {
            assert!(is_exploded, "a list of numbers repeats its term");
        }

        shape
    }

    fn specified_of(self, located: Located<'spec>) -> Specified<'spec> {
        assert!(located.operation.is_object(), "an operation is an object");

        let Located { item, method, operation, path } = located;
        let identifier = operation.get("operationId").and_then(Value::as_str).expect("an id");
        let shared = item.get("parameters").and_then(Value::as_array).into_iter().flatten();
        let own = operation.get("parameters").and_then(Value::as_array).into_iter().flatten();
        let mut parameters: Vec<Declared<'spec>> = Vec::new();
        let mut is_xsrf = operation.to_string().contains(HEADER_XSRF.0);

        for raw in shared.chain(own) {
            let parameter = self.resolved(raw);

            if parameter.get("name").and_then(Value::as_str) == Some(HEADER_XSRF.0) {
                is_xsrf = true;
            } else {
                parameters.push(retyped(identifier, self.declared_of(parameter)));
            }
        }

        let responses = operation.get("responses").and_then(Value::as_object).expect("responses");
        let success = self.success_of(responses);
        let schema = success.and_then(|(_status, response)| self.schema_of(response));
        let has_content = success.is_some_and(|(_status, response)| response_has_content(response));
        let walked = schema.and_then(|found| self.walked_of(&parameters, found));

        let reply = match (success, schema) {
            (None, _) => Reply::Binary,
            (Some(_), Some(_)) => Reply::JSONValue,
            (Some(_), None) if has_content => Reply::Binary,
            (Some(_), None) => Reply::Nothing,
        };

        assert!(parameters.len() <= VALUE_COUNT_MAX, "an operation takes few parameters");

        Specified {
            body: operation.get("requestBody").map(|raw| self.body_of(raw)),
            identifier,
            is_xsrf,
            method,
            parameters,
            path: format!("{}{path}", self.base()),
            reply,
            reply_status: success.map_or(200, |(status, _response)| status),
            reply_value: schema.map_or(Value::Null, |found| self.sample_of(found)),
            walked,
        }
    }

    fn specified_read(self) -> Vec<Specified<'spec>> {
        assert!(self.root.is_object(), "a specification is an object");

        let paths = self.root.get("paths").and_then(Value::as_object).expect("paths");
        let mut specified: Vec<Specified<'spec>> = Vec::new();

        for (path, item) in paths {
            let found = METHODS.into_iter().filter_map(|method| Some((method, item.get(method)?)));

            for (method, operation) in found {
                specified.push(self.specified_of(Located { item, method, operation, path }));
            }
        }

        assert!(!specified.is_empty(), "a specification names operations");

        specified
    }

    fn success_of(self, responses: &'spec Map<String, Value>) -> Option<(u16, &'spec Value)> {
        assert!(!responses.is_empty(), "an operation names its responses");

        let successes = responses.iter().filter(|(code, _raw)| code.starts_with('2'));
        let (code, raw) = successes.min_by(|left, right| left.0.cmp(right.0))?;
        let status = code.parse::<u16>().expect("a status code");

        assert!((200..=299).contains(&status), "a success is a 2xx status");

        Some((status, self.resolved(raw)))
    }

    fn walked_of(self, parameters: &[Declared<'spec>], schema: &'spec Value) -> Option<Walked> {
        assert!(matches!(schema, Value::Object(_) | Value::Null), "a schema is an object");

        let members = self.members_of(schema);
        let queries = parameters.iter().filter(|found| found.place == Place::Query);
        let wires: Vec<&str> = queries.map(|found| found.wire).collect();
        let cursor = ["cursor", "start"].into_iter().find(|wire| wires.contains(wire))?;

        let items = ["results", "data"].into_iter().find(|name| {
            members.get(name).and_then(|found| found.get("type")) == Some(&json!("array"))
        })?;

        let next = if members.contains_key("_links") {
            Next::Link("_links")
        } else if members.contains_key("links") {
            Next::Link("links")
        } else if members.contains_key("cursor") {
            Next::Token
        } else {
            return None;
        };

        let item = members.get(items).and_then(|found| found.get("items")).unwrap_or(&Value::Null);

        assert!(!cursor.is_empty(), "a walk names its cursor");

        Some(Walked { cursor, item: self.sample_of(item), items, next })
    }
}

fn answer_of(specified: &Specified<'_>) -> Scripted {
    assert!((200..=299).contains(&specified.reply_status), "an operation answers a success");

    let response = match specified.reply {
        Reply::Binary => {
            let base = fake::response(specified.reply_status, &Value::Null);

            Response { body: DOWNLOAD_BYTES.to_vec(), ..base }
        }
        Reply::JSONValue => {
            let base = fake::response(specified.reply_status, &specified.reply_value);

            Response { location: Some(LOCATION_ANSWERED.to_owned()), ..base }
        }
        Reply::Nothing => {
            let base = fake::response(specified.reply_status, &Value::Null);

            Response { location: Some(LOCATION_ANSWERED.to_owned()), ..base }
        }
    };

    assert_eq!(response.status, specified.reply_status, "the answer carries the declared status");

    Scripted::Answered(response)
}

fn body_failure(plan: &Value, seen: &Seen, method: &str) -> Option<String> {
    assert!(plan.is_object(), "a plan is an object");
    assert!(METHODS.contains(&method), "the spec names the method");

    if let Some(body) = plan.get("body") {
        let sent: Option<Value> = serde_json::from_slice(&seen.body).ok();

        if seen.content_type.as_deref() != Some(CONTENT_JSON) {
            return Some(format!("a JSON body went as {:?}", seen.content_type));
        }

        return (sent.as_ref() != Some(body)).then(|| format!("the body went as {sent:?}"));
    }

    if let Some(parts) = plan.get("parts").and_then(Value::as_array) {
        return parts_failure(parts, seen);
    }

    let bare_type = if method == "get" { None } else { Some(CONTENT_JSON) };
    let is_bare = seen.content_type.as_deref() == bare_type && seen.body.is_empty();

    assert!(plan.get("parts").is_none(), "parts are a list");

    (!is_bare).then(|| format!("a body nobody declared went as {:?}", seen.content_type))
}

fn body_holds(seen: &Seen, needle: &str) -> bool {
    assert!(!needle.is_empty(), "a search looks for something");
    assert!(seen.body.len() <= BODY_BYTES_MAX, "a sweep body is small");

    seen.body.windows(needle.len()).any(|window| window == needle.as_bytes())
}

fn body_sample(identifier: &str, body: Body) -> (&'static str, Value) {
    assert!(!identifier.is_empty(), "a body belongs to an operation");

    if !body.is_multipart {
        return ("body", json!({"identifier": identifier, "text": format!("body{TEXT_SALT}")}));
    }

    let file = json!({
        "content_type": "text/plain",
        "file_name": "sweep.txt",
        "name": "file",
        "text": format!("file of {identifier}"),
    });

    let parts = json!([file, {"name": "comment", "text": "a comment"}]);

    assert!(parts.as_array().is_some_and(|listed| listed.len() == 2), "a file and a comment");

    ("parts", parts)
}

fn decoded(text: &str) -> String {
    assert!(text.len() <= URL_BYTES_MAX, "a URL piece fits a URL");

    let mut pieces = text.split('%');
    let mut bytes: Vec<u8> = pieces.next().unwrap_or_default().as_bytes().to_vec();

    for piece in pieces {
        let split = piece.split_at_checked(2);
        let escape = split.and_then(|(digits, _rest)| u8::from_str_radix(digits, 16).ok());

        if let Some(byte) = escape {
            bytes.push(byte);
            bytes.extend_from_slice(split.map_or("", |(_digits, rest)| rest).as_bytes());
        } else {
            bytes.push(b'%');
            bytes.extend_from_slice(piece.as_bytes());
        }
    }

    assert!(bytes.len() <= text.len(), "decoding never lengthens a text");

    String::from_utf8(bytes).unwrap_or_else(|failure| format!("<not UTF-8: {failure}>"))
}

fn escape_is_whole(piece: &str) -> bool {
    assert!(piece.len() <= URL_BYTES_MAX, "a URL piece fits a URL");

    let Some((digits, rest)) = piece.split_at_checked(2) else {
        return false;
    };

    if !digits.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return false;
    }

    text_is_unreserved(rest)
}

#[test]
fn every_operation_sends_what_its_specification_declares_and_reads_its_reply() {
    let tables: [(&str, Table<'_>); 3] = [
        ("automation", &<FakeNetwork as automation::Operations>::ALL),
        ("v1", &<FakeNetwork as v1::Operations>::ALL),
        ("v2", &<FakeNetwork as v2::Operations>::ALL),
    ];

    let mut failures: Vec<String> = Vec::new();
    let mut operation_count = 0_u64;
    let mut walk_count = 0_u64;

    for (module, table) in tables {
        let root = document_read(module);
        let runs: BTreeMap<&str, Run<FakeNetwork>> = table.iter().copied().collect();
        let operations = Document { root: &root }.specified_read();

        assert_eq!(runs.len(), operations.len(), "{module} sweeps each operation once");
        assert_eq!(runs.len(), table.len(), "{module} names each sweep function once");

        for specified in &operations {
            let run = *runs.get(specified.identifier).expect("a sweep function per operation");
            let found = operation_failures(Swept { run, specified });

            failures.extend(found.into_iter().map(|text| format!("{module} {text}")));
            operation_count = operation_count.strict_add(1);
            walk_count = walk_count.strict_add(u64::from(specified.walked.is_some()));
        }
    }

    let shown_count = usize::from(FAILURE_SHOWN_COUNT_MAX);
    let shown: Vec<&str> = failures.iter().take(shown_count).map(String::as_str).collect();

    assert!(failures.is_empty(), "{} failures:\n{}", failures.len(), shown.join("\n"));
    assert_eq!(operation_count, OPERATION_COUNT_TOTAL, "every operation of every product ran");
    assert_eq!(walk_count, PAGED_COUNT_TOTAL, "every operation the specifications page walked");
}

fn filled(segment: &str, specified: &Specified<'_>, values: &Map<String, Value>) -> String {
    assert!(segment.contains('{'), "only a segment with a placeholder is filled");

    let paths = specified.parameters.iter().filter(|found| found.place == Place::Path);
    let mut text = segment.to_owned();

    for declared in paths {
        let wire = declared.wire;

        if let Some(value) = values.get(wire) {
            text = text.replace(&format!("{{{wire}}}"), &value_text(value));
        }
    }

    assert!(text.len() <= URL_BYTES_MAX, "a filled segment fits a URL");

    text
}

fn fnv(text: &str) -> u64 {
    assert!(!text.is_empty(), "a sentinel is seeded by a name");

    let mut hash = FNV_OFFSET;

    for byte in text.bytes() {
        hash = (hash ^ u64::from(byte)).wrapping_mul(FNV_PRIME);
    }

    assert_ne!(hash, FNV_OFFSET, "a name moves the hash");

    hash
}

fn header_failure(
    specified: &Specified<'_>,
    values: &Map<String, Value>,
    seen: &Seen,
) -> Option<String> {
    assert!(seen.headers.len() <= HEADER_COUNT_MAX, "a request carries few headers");

    let headers = specified.parameters.iter().filter(|found| found.place == Place::Header);
    let mut expected: Vec<(String, String)> = Vec::new();

    for declared in headers {
        if let Some(value) = values.get(declared.wire) {
            expected.push((declared.wire.to_owned(), value_text(value)));
        }
    }

    if specified.is_xsrf {
        expected.push((HEADER_XSRF.0.to_owned(), HEADER_XSRF.1.to_owned()));
    }

    let mut found = seen.headers.clone();

    expected.sort();
    found.sort();

    assert!(expected.len() <= HEADER_COUNT_MAX, "a plan sends few headers");

    (expected != found).then(|| format!("the headers went as {found:?}, declared {expected:?}"))
}

fn integer_of(wire: &str) -> i64 {
    assert!(!wire.is_empty(), "a sentinel is seeded by a name");

    let bounded = fnv(wire).strict_rem(1_000_000).strict_add(1);
    let integer = i64::try_from(bounded).unwrap_or(1);

    assert!(integer >= 1, "a sentinel number is positive");
    assert!(integer <= 1_000_000, "a sentinel number is short");

    integer
}

fn media_is_json(media: &str) -> bool {
    assert!(!media.is_empty(), "a media type is named");
    assert!(media.contains('/'), "a media type names a family and a kind");

    if media == "*/*" {
        return true;
    }

    media.contains("json")
}

fn operation_failures(swept: Swept<'_>) -> Vec<String> {
    assert!(!swept.specified.identifier.is_empty(), "an operation is named");

    let identifier = swept.specified.identifier;
    let mut failures: Vec<String> = Vec::new();

    for mode in [Mode::Full, Mode::Required] {
        let found = send_failures(swept, mode);

        failures.extend(found.into_iter().map(|text| format!("{identifier} {mode:?}: {text}")));
    }

    let walked = walk_failures(swept);

    failures.extend(walked.into_iter().map(|text| format!("{identifier} Walk: {text}")));

    assert!(failures.iter().all(|text| text.starts_with(identifier)), "each failure is named");

    failures
}

fn page_of(walked: &Walked, next: Option<&str>) -> Value {
    assert!(!walked.items.is_empty(), "a page names its items");

    let mut page = Map::new();

    let _items = page.insert(walked.items.to_owned(), json!([walked.item.clone()]));

    let _next = match (walked.next, next) {
        (Next::Link(container), Some(found)) => {
            page.insert(container.to_owned(), json!({"next": found}))
        }
        (Next::Link(container), None) => page.insert(container.to_owned(), json!({})),
        (Next::Token, Some(found)) => page.insert("cursor".to_owned(), json!(found)),
        (Next::Token, None) => None,
    };

    assert!(page.contains_key(walked.items), "a page holds its items");

    Value::Object(page)
}

fn parts_failure(parts: &[Value], seen: &Seen) -> Option<String> {
    assert!(!parts.is_empty(), "a multipart plan holds a part");

    let content_type = seen.content_type.as_deref().unwrap_or_default();

    let Some(boundary) = content_type.strip_prefix(CONTENT_MULTIPART) else {
        return Some(format!("a multipart body went as {content_type:?}"));
    };

    let opener = format!("--{boundary}\r\n");

    if !seen.body.starts_with(opener.as_bytes()) {
        return Some(format!("a multipart body never opens its boundary {boundary}"));
    }

    for part in parts {
        let name_text = part.get("name").and_then(Value::as_str).unwrap_or_default();
        let name = format!("name=\"{name_text}\"");
        let text = part.get("text").and_then(Value::as_str).unwrap_or_default();

        if !body_holds(seen, &name) {
            return Some(format!("a multipart body lost the part {name}"));
        }

        if !body_holds(seen, text) {
            return Some(format!("a multipart body lost the text of {name}"));
        }
    }

    assert!(!boundary.is_empty(), "a multipart body names its boundary");

    None
}

fn path_failures(
    specified: &Specified<'_>,
    values: &Map<String, Value>,
    sent: &str,
) -> Vec<String> {
    let declared: Vec<&str> = specified.path.split('/').collect();
    let segments: Vec<&str> = sent.split('/').collect();

    if declared.len() != segments.len() {
        return vec![format!("the path went as {sent}, declared {}", specified.path)];
    }

    let mut failures: Vec<String> = Vec::new();

    for (template, segment) in declared.iter().zip(&segments) {
        if !template.contains('{') {
            if template != segment {
                failures.push(format!("the path segment {segment} stands where {template} is"));
            }

            continue;
        }

        let expected = filled(template, specified, values);

        if !raw_is_escaped(segment) {
            failures.push(format!("the path segment {segment} carries a reserved character"));
        }

        if decoded(segment) != expected {
            failures.push(format!("the path segment {segment} reads as other than {expected:?}"));
        }
    }

    assert!(failures.len() <= declared.len().strict_mul(2), "two failures per segment at most");

    failures
}

fn plan_of(specified: &Specified<'_>, mode: Mode) -> Value {
    let mut plan = Map::new();
    let mut values = Map::new();

    for declared in &specified.parameters {
        let is_sent = match mode {
            Mode::Required => declared.is_required,
            Mode::Full | Mode::Walk => true,
        };

        if is_sent {
            let _replaced = values.insert(declared.wire.to_owned(), sentinel_of(declared));
        }
    }

    let _identifier = plan.insert("identifier".to_owned(), json!(specified.identifier));
    let _values = plan.insert("values".to_owned(), Value::Object(values));

    if let Some(body) = specified.body {
        let is_sent = match mode {
            Mode::Required => body.is_required,
            Mode::Full | Mode::Walk => true,
        };

        if is_sent {
            let (key, sample) = body_sample(specified.identifier, body);

            let _body = plan.insert(key.to_owned(), sample);
        }
    }

    if mode == Mode::Walk {
        let _walk = plan.insert("walk".to_owned(), json!(PAGE_COUNT_WALKED));
    }

    assert!(plan.contains_key("values"), "a plan holds its values");
    assert!(plan.len() <= 4, "a plan holds an identifier, values, a body, and a walk at most");

    Value::Object(plan)
}

fn query_failure(
    specified: &Specified<'_>,
    values: &Map<String, Value>,
    query: &str,
) -> Option<String> {
    assert!(query.len() <= URL_BYTES_MAX, "a query fits a URL");

    for term in query.split('&').filter(|piece| !piece.is_empty()) {
        let (name, value) = term.split_once('=').unwrap_or((term, ""));

        if !raw_is_escaped(name) {
            return Some(format!("the query name {name} carries a reserved character"));
        }

        if !value.split(',').all(raw_is_escaped) {
            return Some(format!("the query value {value} carries a reserved character"));
        }
    }

    let expected = terms_expected(specified, values);
    let found = terms_found(specified, query);

    assert!(expected.len() <= QUERY_TERM_COUNT_MAX, "a request carries few terms");

    (expected != found).then(|| format!("the query went as {found:?}, declared {expected:?}"))
}

fn raw_is_escaped(raw: &str) -> bool {
    assert!(raw.len() <= URL_BYTES_MAX, "a URL piece fits a URL");

    let mut pieces = raw.split('%');

    if !text_is_unreserved(pieces.next().unwrap_or_default()) {
        return false;
    }

    pieces.all(escape_is_whole)
}

fn refusal_failures(swept: Swept<'_>, step: &Step<'_>) -> Vec<String> {
    assert!(swept.specified.walked.is_none(), "only an operation that never pages refuses a walk");
    assert!(step.is_walk(), "a refusal answers a walk");

    let client = fake::client(vec![answer_of(swept.specified)]);
    let sent = (swept.run)(&client, step);
    let seen = client.network().seen();
    let mut failures: Vec<String> = Vec::new();

    if sent.is_ok() {
        failures.push("an operation the specification does not page walked".to_owned());
    }

    if !seen.is_empty() {
        failures.push("a walk it refused still sent a request".to_owned());
    }

    failures
}

fn reply_failure(specified: &Specified<'_>, sent: &Sent) -> Option<String> {
    assert!(sent.pages.is_empty(), "a single reply walks no pages");
    assert_eq!(sent.item_count, 0, "a single reply counts no items");

    let location = sent.location.as_deref();

    match specified.reply {
        Reply::Binary if sent.bytes != DOWNLOAD_BYTES => Some("the download lost bytes".to_owned()),
        Reply::Binary => location.map(|found| format!("a download kept the location {found}")),
        Reply::JSONValue | Reply::Nothing if location != Some(LOCATION_ANSWERED) => {
            Some(format!("the reply lost its location, kept {location:?}"))
        }
        Reply::JSONValue if !sent.is_typed => {
            Some(format!("the reply {} reads as no kind", sent.value))
        }
        Reply::JSONValue => {
            (sent.value != specified.reply_value).then(|| "the reply changed".to_owned())
        }
        Reply::Nothing => (!sent.value.is_null()).then(|| "an empty reply held a value".to_owned()),
    }
}

fn request_failures(specified: &Specified<'_>, plan: &Value, seen: &Seen) -> Vec<String> {
    assert!(seen.url.starts_with(GATEWAY), "every request goes through the gateway");

    let values = plan.get("values").and_then(Value::as_object).expect("a plan holds its values");
    let (path, query) = url_split(&seen.url);
    let accept = if specified.reply == Reply::Binary { ACCEPT_ANY } else { ACCEPT_JSON };
    let is_read = specified.method == "get" || READS_BY_POST.contains(&specified.identifier);
    let mut failures = path_failures(specified, values, path);

    if !seen.method.text().eq_ignore_ascii_case(specified.method) {
        failures.push(format!("the method went as {}", seen.method.text()));
    }

    if seen.is_read != is_read {
        failures.push(format!("the request reads as a read {}", seen.is_read));
    }

    if seen.accept != accept {
        failures.push(format!("the request accepts {}", seen.accept));
    }

    failures.extend(query_failure(specified, values, query));
    failures.extend(header_failure(specified, values, seen));
    failures.extend(body_failure(plan, seen, specified.method));

    assert!(failures.len() <= FAILURE_COUNT_MAX, "a request fails in a bounded number of ways");

    failures
}

fn required_failure(specified: &Specified<'_>, step: &Step<'_>) -> Option<String> {
    assert!(step.is_spent(), "the sweep read every value of the plan");

    let required = specified.parameters.iter().filter(|found| found.is_required);
    let mut declared: Vec<&str> = required.map(|found| found.wire).collect();

    if let Some(body) = specified.body.filter(|found| found.is_required) {
        declared.push(if body.is_multipart { WIRE_PARTS } else { WIRE_BODY });
    }

    declared.sort_unstable();

    let read = step.required();

    assert!(declared.windows(2).all(|pair| pair.first() != pair.last()), "each input once");

    (read != declared).then(|| format!("the operation needs {read:?}, declared {declared:?}"))
}

fn response_has_content(response: &Value) -> bool {
    assert!(response.is_object(), "a response is an object");

    let content = response.get("content").and_then(Value::as_object);
    let has_content = content.is_some_and(|media| !media.is_empty());

    assert!(content.is_some() || !has_content, "content is only ever where it is named");

    has_content
}

fn retyped<'spec>(identifier: &str, declared: Declared<'spec>) -> Declared<'spec> {
    assert!(!identifier.is_empty(), "an operation is named");
    assert!(!declared.wire.is_empty(), "a parameter is named");

    if !INTEGERS_READ_AS_TEXT.contains(&(identifier, declared.wire)) {
        return declared;
    }

    assert_eq!(declared.shape, Shape::Integer, "{identifier} declares {} a number", declared.wire);

    Declared { shape: Shape::Text, ..declared }
}

fn send_failures(swept: Swept<'_>, mode: Mode) -> Vec<String> {
    assert!(!swept.specified.identifier.is_empty(), "an operation is named");
    assert_ne!(mode, Mode::Walk, "a send is no walk");

    let specified = swept.specified;
    let plan = plan_of(specified, mode);
    let step = Step::of(&plan).expect("a plan the sweep built");
    let client = fake::client(vec![answer_of(specified)]);
    let sent = (swept.run)(&client, &step);
    let seen = client.network().seen();
    let mut failures: Vec<String> = Vec::new();

    if let [request] = seen.as_slice() {
        failures.extend(request_failures(specified, &plan, request));
    } else {
        failures.push(format!("{} requests went out for one send", seen.len()));
    }

    failures.extend(required_failure(specified, &step));

    match sent {
        Ok(found) => failures.extend(reply_failure(specified, &found)),
        Err(cause) => failures.push(format!("the send failed: {cause}")),
    }

    assert!(step.is_spent(), "the sweep read every value of the plan");

    failures
}

fn sentinel_of(declared: &Declared<'_>) -> Value {
    let wire = declared.wire;
    let seed = fnv(wire);

    assert!(!wire.contains([',', '|']), "a wire name holds no list separator");
    assert!(!TEXT_SALT.contains([',', '|']), "a sentinel holds no list separator");

    match declared.shape {
        Shape::Boolean => json!(seed.is_multiple_of(2)),
        Shape::Decimal => json!(seed.strict_rem(90).strict_add(10).to_string()),
        Shape::Integer => json!(integer_of(wire)),
        Shape::Integers => json!([integer_of(wire), integer_of(&format!("{wire} two"))]),
        Shape::Pairs => json!({"k1": format!("{wire} one&"), "k2": "two/#="}),
        Shape::Text if declared.place == Place::Header => json!(format!("\"etag-{wire}\"")),
        Shape::Text => json!(format!("{wire}{TEXT_SALT}")),
        Shape::Texts | Shape::TextsJoined => {
            json!([format!("{wire} one{TEXT_SALT}"), format!("{wire} two")])
        }
    }
}

fn terms_expected(specified: &Specified<'_>, values: &Map<String, Value>) -> Vec<(String, String)> {
    assert!(values.len() <= VALUE_COUNT_MAX, "a plan holds few values");

    let queries = specified.parameters.iter().filter(|found| found.place == Place::Query);
    let mut terms: Vec<(String, String)> = Vec::new();

    for declared in queries {
        if let Some(value) = values.get(declared.wire) {
            terms.extend(terms_of(declared, value));
        }
    }

    terms.sort();

    assert!(terms.len() <= QUERY_TERM_COUNT_MAX, "a request carries few terms");

    terms
}

fn terms_found(specified: &Specified<'_>, query: &str) -> Vec<(String, String)> {
    assert!(query.len() <= URL_BYTES_MAX, "a query fits a URL");

    let mut terms: Vec<(String, String)> = Vec::new();

    for term in query.split('&').filter(|piece| !piece.is_empty()) {
        let (raw_name, raw_value) = term.split_once('=').unwrap_or((term, ""));
        let name = decoded(raw_name);

        let is_joined = specified
            .parameters
            .iter()
            .any(|declared| declared.shape == Shape::TextsJoined && declared.wire == name);

        let value = if is_joined {
            raw_value.split(',').map(decoded).collect::<Vec<String>>().join(LIST_SEPARATOR)
        } else {
            decoded(raw_value)
        };

        terms.push((name, value));
    }

    terms.sort();

    assert!(terms.len() <= QUERY_TERM_COUNT_MAX, "a request carries few terms");

    terms
}

fn terms_of(declared: &Declared<'_>, value: &Value) -> Vec<(String, String)> {
    assert!(!value.is_null(), "a value sent holds something");

    let wire = declared.wire;
    let listed = value.as_array().map_or(&[][..], Vec::as_slice);

    let terms: Vec<(String, String)> = match declared.shape {
        Shape::Integers | Shape::Texts => {
            listed.iter().map(|item| (wire.to_owned(), value_text(item))).collect()
        }
        Shape::TextsJoined => {
            let joined: Vec<String> = listed.iter().map(value_text).collect();

            vec![(wire.to_owned(), joined.join(LIST_SEPARATOR))]
        }
        Shape::Pairs => value
            .as_object()
            .into_iter()
            .flatten()
            .map(|(key, item)| (format!("{wire}[{key}]"), value_text(item)))
            .collect(),
        Shape::Boolean | Shape::Decimal | Shape::Integer | Shape::Text => {
            vec![(wire.to_owned(), value_text(value))]
        }
    };

    assert!(!terms.is_empty(), "a value sent is at least one term");

    terms
}

fn text_is_unreserved(text: &str) -> bool {
    assert!(text.len() <= URL_BYTES_MAX, "a URL piece fits a URL");

    let is_unreserved = text.bytes().all(
        |byte| matches!(byte, b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'-' | b'.' | b'_' | b'~'),
    );

    assert!(text.is_ascii() || !is_unreserved, "an unreserved piece is ASCII");

    is_unreserved
}

fn url_split(url: &str) -> (&str, &str) {
    assert!(url.starts_with(GATEWAY), "every request goes through the gateway");

    let split = url.split_once('?').unwrap_or((url, ""));

    assert!(!split.0.contains('?'), "a path holds no query mark");

    split
}

fn value_text(value: &Value) -> String {
    assert!(!value.is_null(), "a value sent holds something");

    let text = match value {
        Value::Bool(found) => found.to_string(),
        Value::Number(found) => found.to_string(),
        Value::String(found) => found.clone(),
        Value::Array(_) | Value::Null | Value::Object(_) => panic!("{value} is no single value"),
    };

    assert!(!text.is_empty(), "a sentinel is never empty");

    text
}

fn walk_failures(swept: Swept<'_>) -> Vec<String> {
    assert!(!swept.specified.identifier.is_empty(), "an operation is named");

    let specified = swept.specified;
    let plan = plan_of(specified, Mode::Walk);
    let step = Step::of(&plan).expect("a plan the sweep built");

    assert!(step.is_walk(), "a walk plan walks");

    let Some(walked) = specified.walked.as_ref() else {
        return refusal_failures(swept, &step);
    };

    let (link, next) = walk_next(walked);
    let opening = fake::answer(200, &page_of(walked, Some(&link)));
    let client = fake::client(vec![opening, fake::answer(200, &page_of(walked, None))]);
    let sent = (swept.run)(&client, &step);
    let seen = client.network().seen();
    let mut failures: Vec<String> = Vec::new();

    match sent {
        Ok(found) => failures.extend(walk_reply_failure(&found)),
        Err(cause) => failures.push(format!("the walk failed: {cause}")),
    }

    let [opening_seen, closing_seen] = seen.as_slice() else {
        failures.push(format!("{} requests went out for two pages", seen.len()));

        return failures;
    };

    failures.extend(request_failures(specified, &plan, opening_seen));
    failures.extend(walk_follow_failure(specified, (opening_seen, closing_seen), walked, &next));

    assert!(step.is_spent(), "the sweep read every value of the plan");

    failures
}

fn walk_follow_failure(
    specified: &Specified<'_>,
    requests: (&Seen, &Seen),
    walked: &Walked,
    next: &str,
) -> Option<String> {
    assert_eq!(requests.0.method, requests.1.method, "a walk repeats one operation");
    assert!(!next.is_empty(), "a walk goes on to something");

    let (opening, closing) = requests;
    let (opening_path, opening_query) = url_split(&opening.url);
    let (closing_path, closing_query) = url_split(&closing.url);
    let mut expected = terms_found(specified, opening_query);

    expected.retain(|(name, _value)| name != walked.cursor);
    expected.push((walked.cursor.to_owned(), next.to_owned()));
    expected.sort();

    let found = terms_found(specified, closing_query);

    if opening_path != closing_path {
        return Some(format!("the walk moved to {closing_path}"));
    }

    (found != expected).then(|| format!("the walk went on as {found:?}, expected {expected:?}"))
}

fn walk_next(walked: &Walked) -> (String, String) {
    assert!(!walked.items.is_empty(), "a walk names its items");

    let next = match (walked.next, walked.cursor) {
        (Next::Link(_container), "start") => {
            (format!("/wiki/next?limit=25&start={START_NEXT}"), START_NEXT.to_owned())
        }
        (Next::Link(_container), _) => {
            (format!("/wiki/next?limit=25&cursor={CURSOR_LINKED}"), CURSOR_NEXT.to_owned())
        }
        (Next::Token, _) => (CURSOR_NEXT.to_owned(), CURSOR_NEXT.to_owned()),
    };

    assert!(!next.1.is_empty(), "a walk goes on to something");

    next
}

fn walk_reply_failure(sent: &Sent) -> Option<String> {
    assert!(sent.bytes.is_empty(), "a walk downloads nothing");
    assert!(sent.value.is_null(), "a walk keeps its pages, not one value");

    let page_count = u64::try_from(sent.pages.len()).unwrap_or(u64::MAX);

    if page_count != PAGE_COUNT_WALKED {
        return Some(format!("the walk read {page_count} pages"));
    }

    if sent.item_count != page_count {
        return Some(format!("the walk read {} items over {page_count} pages", sent.item_count));
    }

    (!sent.is_typed).then(|| "the walk read items of another kind".to_owned())
}
