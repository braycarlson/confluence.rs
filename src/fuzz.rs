use core::iter;
use core::time::Duration;
use std::io::{Seek, Write};
use std::sync::Mutex;

use serde_json::{Map, Value, json};

use crate::adf;
use crate::automation::SearchManualRulesWithGet;
use crate::client::Client;
use crate::credentials::Credentials;
use crate::encoding::{percent, percent_decode};
use crate::error::Error;
use crate::kind::Text;
use crate::limits;
use crate::operation::Operation;
use crate::page::Page;
use crate::paged::Paged;
use crate::paging::{Next, Paging};
use crate::prng::{Prng, Seed};
use crate::request::Request;
use crate::response::Response;
use crate::size;
use crate::text;
use crate::transport::Transport;
use crate::v1::{GetCurrentUser, GetDescendantsOfType, get_descendants_of_type};
use crate::v2::{CreatePage, GetSpaces, ListSpacePermissionCombinations};
use crate::wait::Wait;

pub const CASE_COUNT_SMOKE: u32 = 1_000;
pub const NAMES: [&str; 5] = ["adf", "client", "page", "text", "url"];
pub const SEED_SMOKE: u64 = 123;
const ATTEMPT_COUNT_MAX: u64 = 5;

const CREDENTIALS: Credentials<'static> =
    Credentials::Token { cloud_id: "fuzz-site", email: "fuzz@example.com", token: "fuzz" };

const DEPTH_MAX: u32 = 10;
const FAULT_STATUSES: [u16; 11] = [200, 201, 204, 303, 400, 401, 404, 429, 429, 500, 503];

const FLATTEN_BUDGET: adf::Budget =
    adf::Budget { node_count_max: 20_000, text_bytes_max: 256 * 1024 };

const GROUP_COUNT_MAX: u64 = 4;
const LEAF_COUNT_MAX: u64 = 12;
const MARKER: &str = "MARK7F3A";

const PAGE_FIELDS: [&str; 7] = ["results", "_links", "cursor", "totalSize", "data", "links", "x"];

const PAGE_FIELD_COUNT_MAX: u64 = 7;
const PAGE_HOSTILE_COUNT_MAX: u64 = 12;

const PAGINGS: [Paging; 4] = [
    Paging { cursor: "cursor", items: "/results", next: Next::Link("/_links/next") },
    Paging { cursor: "start", items: "/results", next: Next::Link("/_links/next") },
    Paging { cursor: "cursor", items: "/data", next: Next::Link("/links/next") },
    Paging { cursor: "cursor", items: "/results", next: Next::Token("/cursor") },
];

const RETRY_AFTER_SECONDS_GUARD: u64 = 900;
const URL_BASE: &str = "https://api.atlassian.com/ex/confluence/site";

const URL_PIECES: [&str; 14] =
    ["/", "..", ".", "?", "&", "=", "#", "%", "%2F", "{id}", " ", "\u{e9}", "\u{1f600}", "a"];

const URL_PIECE_COUNT_MAX: u64 = 8;

const TYPES_BLOCK: [&str; 10] = [
    "paragraph",
    "bulletList",
    "listItem",
    "panel",
    "blockquote",
    "tableRow",
    "taskList",
    "taskItem",
    "expand",
    "media",
];

const TYPES_HOSTILE: [&str; 4] = ["doc", "somethingNew", "mediaSingle", "codeBlock"];
const WIDE_BYTES: u64 = FLATTEN_BUDGET.text_bytes_max.div_euclid(3);
const WIDE_PERCENT: u64 = 4;

const WORD_ALPHABET: [char; 12] =
    ['a', 'B', '9', '_', '-', '.', ' ', '\n', '\t', '\u{e9}', '\u{4e16}', '\u{1f600}'];

const WORD_CHARACTER_COUNT_MAX: u64 = 24;
const _: () = assert!(ATTEMPT_COUNT_MAX >= 2, "a failure is retried at least once");
const _: () = assert!(PAGE_HOSTILE_COUNT_MAX >= 1, "a walk reads at least one hostile page");
const _: () = assert!(CASE_COUNT_SMOKE >= 1, "a smoke run checks at least one case");
const _: () = assert!(DEPTH_MAX >= 1, "a tree folds at least once");
const _: () = assert!(GROUP_COUNT_MAX >= 1, "a fold groups at least one child");
const _: () = assert!(LEAF_COUNT_MAX >= 1, "a document holds at least one leaf");
const _: () = assert!(WIDE_PERCENT <= 100, "a chance is a percentage");
const _: () = assert!(WORD_CHARACTER_COUNT_MAX >= 1, "a word holds at least one character");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Attempted {
    Read,
    Write,
}

#[derive(Clone, Copy, Debug)]
enum Fault {
    Answer { is_near_limit: bool, retry_after_seconds: Option<u64>, status: u16 },
    Dropped,
}

#[derive(Debug)]
struct Faulty {
    answered_count: Mutex<u64>,
    faults: Vec<Fault>,
    waits: Mutex<Vec<Duration>>,
}

#[derive(Debug)]
struct Hostile {
    bodies: Mutex<Vec<Value>>,
}

impl Faulty {
    fn fault_next(&self) -> Result<Fault, Error> {
        let mut answered = self
            .answered_count
            .lock()
            .map_err(|_poisoned| Error::Input("the attempt count is poisoned".to_owned()))?;

        let fault = self.faults.get(size::offset(*answered)).copied().unwrap_or(Fault::Dropped);

        *answered = answered.strict_add(1);
        drop(answered);

        Ok(fault)
    }
}

impl Transport for Faulty {
    fn fetch(&self, request: &Request, bytes_max: u64) -> Result<Response, Error> {
        assert!(bytes_max >= 1, "a body may hold at least one byte");
        assert!(request.url().starts_with("https://"), "every request is encrypted");

        match self.fault_next()? {
            Fault::Answer { is_near_limit, retry_after_seconds, status } => Ok(Response {
                body: b"{}".to_vec(),
                is_near_limit,
                location: None,
                rate_limit_reason: None,
                retry_after_seconds,
                status,
                streamed_count: 0,
            }),
            Fault::Dropped => Err(Error::Transport("the fuzzer dropped the link".to_owned())),
        }
    }

    fn stream<Sink>(&self, request: &Request, _sink: &mut Sink) -> Result<Response, Error>
    where
        Sink: Seek + Write,
    {
        assert!(request.is_read(), "a download is a read");

        Err(Error::Input("a fault run downloads nothing".to_owned()))
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

impl Transport for Hostile {
    fn fetch(&self, request: &Request, bytes_max: u64) -> Result<Response, Error> {
        assert!(bytes_max >= 1, "a body may hold at least one byte");
        assert!(request.is_read(), "a walk only ever reads");

        let mut bodies = self
            .bodies
            .lock()
            .map_err(|_poisoned| Error::Input("the pages are poisoned".to_owned()))?;

        let body =
            if bodies.is_empty() { json!({"data": [], "results": []}) } else { bodies.remove(0) };

        drop(bodies);

        Ok(Response {
            body: serde_json::to_vec(&body).unwrap_or_default(),
            is_near_limit: false,
            location: None,
            rate_limit_reason: None,
            retry_after_seconds: None,
            status: 200,
            streamed_count: 0,
        })
    }

    fn stream<Sink>(&self, request: &Request, _sink: &mut Sink) -> Result<Response, Error>
    where
        Sink: Seek + Write,
    {
        assert!(request.is_read(), "a download is a read");

        Err(Error::Input("a walk downloads nothing".to_owned()))
    }

    fn wait(&self, wait: &Wait<'_>) -> Result<(), Error> {
        assert!(!wait.reason.is_empty(), "a wait always says why");
        assert!(wait.delay.as_millis() <= u128::from(limits::WAIT_MILLIS_MAX), "a wait fits");

        Ok(())
    }
}

#[must_use]
pub fn adf_flatten(seed: u64, case_count_max: u32) -> u32 {
    assert!(case_count_max >= 1, "seed {seed}: a run checks at least one case");

    let mut prng = Prng::from_seed(seed);
    let mut case_count: u32 = 0;
    let bytes_max = FLATTEN_BUDGET.text_bytes_bound();

    for _case_index in 0..case_count_max {
        let is_planted = prng.boolean();

        let document =
            if is_planted { document_planted(&mut prng) } else { document_hostile(&mut prng) };

        let flattened = adf::text_read_within(&document, FLATTEN_BUDGET);
        let is_truncated = flattened.ends_with(adf::TRUNCATION_NOTE);
        let is_marker_due = is_planted && !is_truncated;

        assert!(
            size::of_text(&flattened) <= bytes_max,
            "seed {seed} case {case_count}: the flattened text passed its budget",
        );

        assert!(
            !flattened.starts_with(char::is_whitespace),
            "seed {seed} case {case_count}: the flattened text opens with whitespace",
        );

        if is_marker_due {
            assert!(
                flattened.contains(MARKER),
                "seed {seed} case {case_count}: the planted marker did not survive",
            );
        }

        case_count = case_count.strict_add(1);
    }

    assert_eq!(case_count, case_count_max, "every case ran");

    case_count
}

fn attempt_count_expected(faults: &[Fault], attempted: Attempted) -> u64 {
    assert_eq!(size::of_slice(faults), ATTEMPT_COUNT_MAX, "a script answers every attempt");

    let retried = faults
        .iter()
        .copied()
        .take(size::offset(ATTEMPT_COUNT_MAX.strict_sub(1)))
        .take_while(|fault| fault_is_retried(*fault, attempted));

    let attempt_count = size::of_offset(retried.count()).strict_add(1);

    assert!(attempt_count <= ATTEMPT_COUNT_MAX, "a failure is retried at most four times");

    attempt_count
}

#[must_use]
pub fn client_retry(seed: u64, case_count_max: u32) -> u32 {
    assert!(case_count_max >= 1, "seed {seed}: a run checks at least one case");

    let mut prng = Prng::from_seed(seed);
    let mut case_count: u32 = 0;

    for _case_index in 0..case_count_max {
        let faults: Vec<Fault> = iter::repeat_with(|| fault_draw(&mut prng))
            .take(size::offset(ATTEMPT_COUNT_MAX))
            .collect();

        let attempted = if prng.boolean() { Attempted::Write } else { Attempted::Read };

        retry_run(seed, &faults, attempted);
        walk_run(seed, &mut prng);

        case_count = case_count.strict_add(1);
    }

    assert_eq!(case_count, case_count_max, "every case ran");

    case_count
}

fn document_hostile(prng: &mut Prng) -> Value {
    let leaf_count = prng.index(LEAF_COUNT_MAX).strict_add(1);
    let mut level: Vec<Value> = Vec::with_capacity(size::offset(leaf_count));

    for _leaf_index in 0..leaf_count {
        level.push(node_hostile(prng));
    }

    assert_eq!(size::of_slice(&level), leaf_count, "every leaf is drawn");

    json!({"type": "doc", "content": tree_fold(prng, level, &TYPES_HOSTILE)})
}

fn document_planted(prng: &mut Prng) -> Value {
    let leaf_count = prng.index(LEAF_COUNT_MAX).strict_add(1);
    let marker_index = prng.index(leaf_count);
    let mut level: Vec<Value> = Vec::with_capacity(size::offset(leaf_count));

    assert!(marker_index < leaf_count, "the marker sits on one of the leaves");

    for leaf_index in 0..leaf_count {
        if leaf_index == marker_index {
            level.push(json!({"type": "text", "text": MARKER}));

            continue;
        }

        level.push(json!({"type": "text", "text": word_build(prng)}));
    }

    assert_eq!(size::of_slice(&level), leaf_count, "every leaf is drawn");

    json!({"type": "doc", "content": tree_fold(prng, level, &TYPES_BLOCK)})
}

fn fault_draw(prng: &mut Prng) -> Fault {
    if prng.chance(25) {
        return Fault::Dropped;
    }

    let status = pick(prng, &FAULT_STATUSES);

    let retry_after_seconds = match prng.index(4) {
        0 => Some(prng.index(RETRY_AFTER_SECONDS_GUARD.strict_add(1))),
        1 => Some(RETRY_AFTER_SECONDS_GUARD.strict_add(prng.index(1_000))),
        _ => None,
    };

    Fault::Answer { is_near_limit: prng.chance(20), retry_after_seconds, status }
}

fn fault_is_retried(fault: Fault, attempted: Attempted) -> bool {
    let Fault::Answer { is_near_limit: _, retry_after_seconds, status } = fault else {
        return attempted == Attempted::Read;
    };

    assert!((100..=599).contains(&status), "a scripted answer carries a real status");

    if !fault_status_is_retried(status, attempted) {
        return false;
    }

    retry_after_seconds.is_none_or(|seconds| seconds <= RETRY_AFTER_SECONDS_GUARD)
}

fn fault_outcome_check(
    seed: Seed,
    fault: Fault,
    attempted: Attempted,
    outcome: &Result<(), Error>,
) {
    let Fault::Answer { is_near_limit: _, retry_after_seconds: _, status } = fault else {
        let is_expected = match attempted {
            Attempted::Read => matches!(outcome, Err(Error::Transport(_))),
            Attempted::Write => matches!(outcome, Err(Error::Indefinite(_))),
        };

        assert!(is_expected, "seed {seed}: a dropped {attempted:?} read as {outcome:?}");

        return;
    };

    if status_is_success(status, attempted) {
        assert!(outcome.is_ok(), "seed {seed}: {status} read as {outcome:?}");

        return;
    }

    if status_may_have_landed(status, attempted) {
        let is_indefinite = matches!(outcome, Err(Error::Indefinite(_)));

        assert!(is_indefinite, "seed {seed}: a {status} on a write read as {outcome:?}");

        return;
    }

    let found = outcome.as_ref().err().and_then(Error::status);

    assert_eq!(found, Some(status), "seed {seed}: {status} read as {outcome:?}");
}

const fn fault_status_is_retried(status: u16, attempted: Attempted) -> bool {
    assert!(status >= 100, "a scripted answer carries a real status");

    if status == 429 {
        return true;
    }

    if status < 500 {
        return false;
    }

    matches!(attempted, Attempted::Read)
}

fn link_build(prng: &mut Prng) -> String {
    let term = if prng.boolean() { "cursor" } else { "start" };
    let token = word_build(prng);
    let link = format!("/wiki/api/v2/spaces?limit=5&{term}={}", percent(&token));

    assert!(link.contains('?'), "a next link carries its query");
    assert!(!token.is_empty(), "a drawn token is never empty");

    link
}

fn node_hostile(prng: &mut Prng) -> Value {
    if prng.chance(WIDE_PERCENT) {
        return node_wide(prng);
    }

    match prng.index(14) {
        0 => json!(null),
        1 => json!(7_i64),
        2 => json!("a bare string where a node belongs"),
        3 => json!({"type": "text"}),
        4 => json!({"type": "text", "text": 7_i64}),
        5 => json!({"type": "paragraph", "content": 5_i64}),
        6 => json!({"type": "mention", "attrs": {"text": word_build(prng)}}),
        7 => json!({"type": "text", "text": word_build(prng), "marks": 5_i64}),
        8 => {
            let marks = json!([{"type": "link", "attrs": {"href": 7_i64}}]);

            json!({"type": "text", "text": "x", "marks": marks})
        }
        9 => json!({"type": "date", "attrs": {"timestamp": word_build(prng)}}),
        10 => json!({"type": "date", "attrs": {"timestamp": prng.int().to_string()}}),
        11 => {
            let millis = prng.index(1 << 54).to_string();

            json!({"type": "date", "attrs": {"timestamp": format!("-{millis}")}})
        }
        12 => json!({"type": "expand", "attrs": {"title": word_build(prng)}, "content": 5_i64}),
        _ => json!({"type": "hardBreak"}),
    }
}

fn node_wide(prng: &mut Prng) -> Value {
    let wide = "w".repeat(size::offset(WIDE_BYTES));

    if !prng.boolean() {
        return json!({"type": "text", "text": wide});
    }

    if prng.boolean() {
        return json!({"type": "nestedExpand", "attrs": {"title": wide}});
    }

    let href = word_build(prng);

    json!({
        "type": "text",
        "text": wide,
        "marks": [{"type": "link", "attrs": {"href": href}}],
    })
}

fn page_build(prng: &mut Prng) -> Value {
    if prng.chance(WIDE_PERCENT) {
        return page_member(prng);
    }

    let field_count = prng.index(PAGE_FIELD_COUNT_MAX).strict_add(1);
    let mut page = Map::new();

    for _field_index in 0..field_count {
        let field = pick(prng, &PAGE_FIELDS);

        let _replaced = page.insert(field.to_owned(), page_member(prng));
    }

    assert!(!page.is_empty(), "a page holds at least one member");

    Value::Object(page)
}

fn page_check(parsed: Result<Page<Text>, Error>, seed: u64) {
    assert!(seed < u64::MAX, "a seed is a number a run can report");

    let page = match parsed {
        Ok(page) => page,
        Err(failure) => {
            assert!(matches!(failure, Error::Reply(_)), "seed {seed}: a fault of the reply");

            return;
        }
    };

    assert!(
        size::of_offset(page.items().count()) <= page.count(),
        "seed {seed}: a walk hands back at most what it counted",
    );

    assert_eq!(
        page.is_last(),
        page.token_next().is_none(),
        "seed {seed}: a page goes on exactly when it names where",
    );
}

fn page_member(prng: &mut Prng) -> Value {
    match prng.index(13) {
        0 => json!(null),
        1 => json!(true),
        2 => json!(false),
        3 => json!(-1_i64),
        4 => json!(prng.index(1_000)),
        5 => json!(""),
        6 => json!(word_build(prng)),
        7 => json!([word_build(prng), 7_i64, null]),
        8 => json!({"next": link_build(prng)}),
        9 => json!({"next": word_build(prng)}),
        10 => json!({"base": "https://example.atlassian.net/wiki", "next": 7_i64}),
        11 => json!({"next": "/wiki/api/v2/spaces?cursor=%FF%FE"}),
        _ => json!([]),
    }
}

#[must_use]
pub fn page_parse(seed: u64, case_count_max: u32) -> u32 {
    assert!(case_count_max >= 1, "seed {seed}: a run checks at least one case");

    let mut prng = Prng::from_seed(seed);
    let mut case_count: u32 = 0;

    for _case_index in 0..case_count_max {
        let reply = page_build(&mut prng);
        let paging = pick_paging(&mut prng);

        page_check(Page::<Text>::parse(reply, paging), seed);

        case_count = case_count.strict_add(1);
    }

    assert_eq!(case_count, case_count_max, "every case ran");

    case_count
}

fn pick<Item>(prng: &mut Prng, items: &[Item]) -> Item
where
    Item: Copy + Default,
{
    assert!(!items.is_empty(), "a pick draws from at least one item");

    let count = size::of_slice(items);
    let index = prng.index(count);

    assert!(index < count, "a pick stays inside its items");

    items.get(size::offset(index)).copied().unwrap_or_default()
}

fn pick_paging(prng: &mut Prng) -> Paging {
    let count = size::of_slice(&PAGINGS);
    let index = prng.index(count);

    assert!(index < count, "a pick stays inside its items");

    let fallback = Paging { cursor: "cursor", items: "/results", next: Next::Token("/cursor") };

    PAGINGS.get(size::offset(index)).copied().unwrap_or(fallback)
}

#[must_use]
pub fn request_url(seed: u64, case_count_max: u32) -> u32 {
    assert!(case_count_max >= 1, "seed {seed}: a run checks at least one case");

    let mut prng = Prng::from_seed(seed);
    let mut case_count: u32 = 0;

    for _case_index in 0..case_count_max {
        let id = url_value_build(&mut prng);
        let kind = url_value_build(&mut prng);
        let depth = url_value_build(&mut prng);
        let operation = GetDescendantsOfType { id: &id, kind: &kind };

        let options = get_descendants_of_type::Options {
            depth: Some(&depth),
            expand: None,
            limit: None,
            start: None,
        };

        let url = format!("{URL_BASE}{}", <GetDescendantsOfType<'_> as Operation>::PATH);
        let mut request = Request::of::<GetDescendantsOfType<'_>>(url, "Basic secret");
        let filled = operation.request(&options, &mut request);

        if let Err(failure) = filled {
            assert!(matches!(failure, Error::Input(_)), "seed {seed}: a caller's fault");
        } else {
            request_url_check(&request, (&id, &kind, &depth), seed);
        }

        case_count = case_count.strict_add(1);
    }

    assert_eq!(case_count, case_count_max, "every case ran");

    case_count
}

fn request_url_check(request: &Request, (id, kind, depth): (&str, &str, &str), seed: u64) {
    assert!(!id.is_empty(), "seed {seed}: a filled id is never empty");

    let url = request.url();
    let tail = url.strip_prefix(URL_BASE).unwrap_or_default();

    assert!(tail.contains('?'), "seed {seed}: a set option always reaches the query");

    let (path, query) = tail.split_once('?').unwrap_or_default();
    let segments: Vec<&str> = path.split('/').collect();

    assert!(request.is_complete(), "seed {seed}: no placeholder survives in {url}");
    assert_eq!(segments.len(), 8, "seed {seed}: a hostile value added a path segment to {url}");
    assert_eq!(query.matches('&').count(), 0, "seed {seed}: a hostile value added a query term");
    assert_eq!(query.matches('?').count(), 0, "seed {seed}: a hostile value added a query mark");

    let decoded_id = percent_decode(segments.get(5).copied().unwrap_or_default());
    let decoded_kind = percent_decode(segments.get(7).copied().unwrap_or_default());
    let decoded_depth = percent_decode(query.trim_start_matches("depth="));

    assert_eq!(decoded_id, id.as_bytes(), "seed {seed}: the id round trips");
    assert_eq!(decoded_kind, kind.as_bytes(), "seed {seed}: the type round trips");
    assert_eq!(decoded_depth, depth.as_bytes(), "seed {seed}: the option round trips");
}

fn retry_check(seed: Seed, faults: &[Fault], attempted: Attempted, network: &Faulty) {
    assert!(!faults.is_empty(), "seed {seed}: a script answers at least once");

    let attempt_count = network.answered_count.lock().map_or(0, |count| *count);
    let attempt_expected = attempt_count_expected(faults, attempted);
    let waits = network.waits.lock().map_or_else(|_poisoned| Vec::new(), |found| found.clone());
    let is_expected = attempt_count == attempt_expected;

    assert!(is_expected, "seed {seed}: {faults:?} on a {attempted:?} ran {attempt_count} attempts");

    let retry_count = attempt_expected.strict_sub(1);

    assert!(
        size::of_slice(&waits) >= retry_count,
        "seed {seed}: {retry_count} retries took fewer waits than that",
    );

    for (fault, wait) in faults.iter().zip(&waits).take(size::offset(retry_count)) {
        let floor = match fault {
            Fault::Answer { is_near_limit: _, retry_after_seconds: Some(seconds), status: _ } => {
                *seconds
            }
            Fault::Answer { is_near_limit: _, retry_after_seconds: None, status: _ }
            | Fault::Dropped => 0,
        };

        assert!(wait.as_secs() >= floor, "seed {seed}: a wait of {wait:?} undercut {floor}");

        assert!(
            wait.as_millis() <= u128::from(limits::WAIT_MILLIS_MAX),
            "seed {seed}: a wait of {wait:?} ran past its cap",
        );
    }
}

fn retry_run(seed: Seed, faults: &[Fault], attempted: Attempted) {
    assert!(!faults.is_empty(), "seed {seed}: a script answers at least once");

    let network = Faulty {
        answered_count: Mutex::new(0),
        faults: faults.to_vec(),
        waits: Mutex::new(Vec::new()),
    };

    let built = Client::new(network, &CREDENTIALS);

    assert!(built.is_ok(), "seed {seed}: the fixture credentials are sound");

    let Ok(client) = built else {
        return;
    };

    let outcome = match attempted {
        Attempted::Read => client.send(&GetCurrentUser).map(|_reply| ()),
        Attempted::Write => {
            let body = json!({"spaceId": "1", "title": "fuzz"});

            client.send(&CreatePage { body: &body }).map(|_reply| ())
        }
    };

    let attempt_expected = attempt_count_expected(faults, attempted);
    let last = faults.get(size::offset(attempt_expected.strict_sub(1))).copied();

    retry_check(seed, faults, attempted, client.network());

    assert!(last.is_some(), "seed {seed}: a script answers every attempt");

    let Some(fault) = last else {
        return;
    };

    fault_outcome_check(seed, fault, attempted, &outcome);
}

#[must_use]
pub fn run(name: &str, seed: u64, case_count: u32) -> Option<u32> {
    assert!(case_count >= 1, "fuzzer {name} at seed {seed}: a run checks at least one case");

    let ran = match name {
        "adf" => Some(adf_flatten(seed, case_count)),
        "client" => Some(client_retry(seed, case_count)),
        "page" => Some(page_parse(seed, case_count)),
        "text" => Some(text_prefix(seed, case_count)),
        "url" => Some(request_url(seed, case_count)),
        _ => None,
    };

    assert_eq!(ran.is_some(), NAMES.contains(&name), "the registry names exactly what runs");
    assert!(ran.is_none_or(|count| count == case_count), "a fuzzer runs every case it was asked");

    ran
}

fn status_is_success(status: u16, attempted: Attempted) -> bool {
    assert!(status >= 100, "a scripted answer carries a real status");

    if (200..=299).contains(&status) {
        return true;
    }

    if status != 303 {
        return false;
    }

    attempted == Attempted::Write
}

const fn status_may_have_landed(status: u16, attempted: Attempted) -> bool {
    assert!(status >= 100, "a scripted answer carries a real status");

    if status < 500 {
        return false;
    }

    matches!(attempted, Attempted::Write)
}

#[must_use]
pub fn text_prefix(seed: u64, case_count_max: u32) -> u32 {
    assert!(case_count_max >= 1, "seed {seed}: a run checks at least one case");

    let mut prng = Prng::from_seed(seed);
    let mut case_count: u32 = 0;

    for _case_index in 0..case_count_max {
        let raw = word_build(&mut prng);
        let raw_bytes = size::of_text(&raw);
        let floor_limit = prng.index(raw_bytes.strict_add(1));
        let boundary = text::boundary_floor(&raw, floor_limit);

        assert!(raw.is_char_boundary(size::offset(boundary)), "seed {seed}: a whole character");
        assert!(boundary <= floor_limit, "seed {seed}: the boundary stays under its limit");

        let cut_limit = text::CODE_POINT_BYTES_MAX.strict_add(prng.index(raw_bytes.strict_add(1)));
        let cut = text::prefix(&raw, cut_limit);

        assert!(size::of_text(cut) <= cut_limit, "seed {seed}: the cut stays under its limit");
        assert!(raw.starts_with(cut), "seed {seed}: the cut is a prefix of the text");

        case_count = case_count.strict_add(1);
    }

    assert_eq!(case_count, case_count_max, "every case ran");

    case_count
}

fn tree_fold(prng: &mut Prng, leaves: Vec<Value>, types: &[&str]) -> Vec<Value> {
    assert!(!leaves.is_empty(), "a fold starts from at least one leaf");
    assert!(!types.is_empty(), "a fold has a node type to fold into");

    let mut level = leaves;

    for _depth_index in 0..DEPTH_MAX {
        if level.len() <= 1 {
            break;
        }

        let level_count = size::of_slice(&level);
        let mut next: Vec<Value> = Vec::with_capacity(level.len());
        let mut taken: u64 = 0;

        for _group_index in 0..level_count {
            if taken >= level_count {
                break;
            }

            let room = level_count.strict_sub(taken);
            let group = prng.index(GROUP_COUNT_MAX).strict_add(1).min(room);

            let children: Vec<Value> =
                level.iter().skip(size::offset(taken)).take(size::offset(group)).cloned().collect();

            let kind = pick(prng, types);

            next.push(json!({"type": kind, "content": children}));

            taken = taken.strict_add(group);
        }

        assert_eq!(taken, level_count, "a fold takes every node once");

        level = next;
    }

    assert!(!level.is_empty(), "a fold keeps at least one node");

    level
}

fn url_value_build(prng: &mut Prng) -> String {
    let piece_count = prng.index(URL_PIECE_COUNT_MAX).strict_add(1);
    let mut value = String::new();

    for _piece_index in 0..piece_count {
        value.push_str(pick(prng, &URL_PIECES));
    }

    assert!(!value.is_empty(), "a value holds at least one piece");
    assert!(piece_count <= URL_PIECE_COUNT_MAX, "a value holds at most its piece budget");

    value
}

fn walk_read<Walked>(seed: Seed, bodies: Vec<Value>, operation: &Walked)
where
    Walked: Paged,
{
    assert!(!bodies.is_empty(), "seed {seed}: a walk reads at least one page");

    let page_count_max = size::of_slice(&bodies).strict_add(1);
    let network = Hostile { bodies: Mutex::new(bodies) };
    let built = Client::new(network, &CREDENTIALS);

    assert!(built.is_ok(), "seed {seed}: the fixture credentials are sound");

    let Ok(client) = built else {
        return;
    };

    let mut walk = client.walk(operation);
    let mut read_count: u64 = 0;

    for _page_index in 0..page_count_max {
        let Some(Ok(_page)) = walk.next() else {
            break;
        };

        read_count = read_count.strict_add(1);
    }

    let following = walk.next();
    let is_over = following.is_none_or(|page| page.is_err() || read_count == page_count_max);

    assert!(read_count <= page_count_max, "seed {seed}: a walk read past its script");
    assert!(is_over, "seed {seed}: a walk went on past its last page");
}

fn walk_run(seed: Seed, prng: &mut Prng) {
    let body_count = prng.index(PAGE_HOSTILE_COUNT_MAX).strict_add(1);

    let bodies: Vec<Value> =
        iter::repeat_with(|| page_build(prng)).take(size::offset(body_count)).collect();

    assert_eq!(size::of_slice(&bodies), body_count, "every hostile page is drawn");

    walk_read(seed, bodies.clone(), &GetSpaces);
    walk_read(seed, bodies.clone(), &ListSpacePermissionCombinations);
    walk_read(seed, bodies.clone(), &SearchManualRulesWithGet { cursor: "AAAABBBBCCCC" });
    walk_read(seed, bodies, &GetDescendantsOfType { id: "1", kind: "page" });
}

fn word_build(prng: &mut Prng) -> String {
    if prng.chance(WIDE_PERCENT) {
        return "w".repeat(size::offset(WIDE_BYTES));
    }

    let character_count = prng.index(WORD_CHARACTER_COUNT_MAX).strict_add(1);
    let mut word = String::with_capacity(size::offset(character_count));

    for _character_index in 0..character_count {
        word.push(pick(prng, &WORD_ALPHABET));
    }

    assert!(!word.is_empty(), "a word holds at least one character");

    assert_eq!(
        size::of_offset(word.chars().count()),
        character_count,
        "a word holds the characters drawn for it",
    );

    word
}

#[cfg(test)]
mod tests {
    use super::{
        CASE_COUNT_SMOKE,
        SEED_SMOKE,
        adf_flatten,
        client_retry,
        document_hostile,
        document_planted,
        page_parse,
        request_url,
        text_prefix,
        word_build,
    };
    use crate::prng::Prng;

    #[test]
    fn a_seed_replays_the_same_inputs_and_another_seed_draws_new_ones() {
        let mut first = Prng::from_seed(7);
        let mut second = Prng::from_seed(7);
        let mut other = Prng::from_seed(8);
        let mut is_identical = true;

        for _case_index in 0..32_u32 {
            let word = word_build(&mut first);
            let planted = document_planted(&mut first);
            let hostile = document_hostile(&mut first);

            assert_eq!(word, word_build(&mut second), "one seed draws one word sequence");
            assert_eq!(planted, document_planted(&mut second), "one seed plants one tree");
            assert_eq!(hostile, document_hostile(&mut second), "one seed draws one hostile tree");

            is_identical &= word == word_build(&mut other);
        }

        assert!(!is_identical, "another seed draws another sequence");
    }

    #[test]
    fn the_adf_flattener_survives_a_smoke_run() {
        assert_eq!(adf_flatten(SEED_SMOKE, CASE_COUNT_SMOKE), CASE_COUNT_SMOKE, "all ran");
    }

    #[test]
    fn the_client_keeps_its_retry_policy_through_a_smoke_run() {
        assert_eq!(client_retry(SEED_SMOKE, CASE_COUNT_SMOKE), CASE_COUNT_SMOKE, "all ran");
    }

    #[test]
    fn the_page_parser_survives_a_smoke_run() {
        assert_eq!(page_parse(SEED_SMOKE, CASE_COUNT_SMOKE), CASE_COUNT_SMOKE, "all ran");
    }

    #[test]
    fn the_request_builder_survives_a_smoke_run() {
        assert_eq!(request_url(SEED_SMOKE, CASE_COUNT_SMOKE), CASE_COUNT_SMOKE, "all ran");
    }

    #[test]
    fn the_text_cutter_survives_a_smoke_run() {
        assert_eq!(text_prefix(SEED_SMOKE, CASE_COUNT_SMOKE), CASE_COUNT_SMOKE, "all ran");
    }
}
