use serde_json::Value;

use crate::size;
use crate::text;

const ATTRIBUTE_KEYS_TEXT: [&str; 5] = ["text", "shortName", "alt", "url", "title"];
const DATE_BYTES_MAX: u64 = 13;
const DATE_ERA_DAYS: i64 = 146_097;
const DATE_ERA_EPOCH_DAYS: i64 = 719_468;
const DATE_MILLIS_LIMIT: u64 = 8_640_000_000_000_000;
const DATE_MILLIS_PER_DAY: i64 = 86_400_000;
const MARK_COUNT_MAX: u64 = 1_024;
const NEWLINE_RUN_MAX: u64 = 2;
const STACK_CAPACITY_INITIAL: u64 = 64;
const TEXT_CAPACITY_INITIAL: u64 = 1024;
pub const TRUNCATION_NOTE: &str = "\n[truncated: document exceeds the flattening budget]";

const TYPES_BLOCK: [&str; 19] = [
    "blockCard",
    "blockquote",
    "bulletList",
    "codeBlock",
    "decisionItem",
    "decisionList",
    "embedCard",
    "heading",
    "listItem",
    "media",
    "mediaGroup",
    "mediaSingle",
    "orderedList",
    "panel",
    "paragraph",
    "rule",
    "tableRow",
    "taskItem",
    "taskList",
];

const TYPES_TITLED: [&str; 2] = ["expand", "nestedExpand"];
const _: () = assert!(DATE_ERA_DAYS == 400 * 365 + 97, "an era is 400 years and 97 leap days");
const _: () = assert!(DATE_MILLIS_PER_DAY == 24 * 60 * 60 * 1000, "a day counts its milliseconds");
const _: () = assert!(NEWLINE_RUN_MAX >= 1, "a paragraph break keeps at least one newline");
const _: () = assert!(Budget::GUARD.is_sound(), "the guard budget keeps every relation");

const _: () = assert!(
    Budget::GUARD.text_bytes_max >= 2 * 25 * 1024 * 1024,
    "a guard sits above twice the 25 MB of the largest ADF field Atlassian allows",
);

const _: () = assert!(
    Budget::GUARD.node_count_max >= Budget::GUARD.text_bytes_max.div_euclid(20),
    "a guard holds a node for every 20 bytes, the smallest node ADF can spell",
);

#[derive(Clone, Copy, Debug)]
pub struct Budget {
    pub node_count_max: u64,
    pub text_bytes_max: u64,
}

#[derive(Clone, Copy)]
enum Step<'document> {
    Break,
    Node(&'document Value),
}

struct Walk<'document> {
    budget: Budget,
    is_truncated: bool,
    stack: Vec<Step<'document>>,
    text: String,
}

impl Budget {
    pub const GUARD: Self = Self { node_count_max: 4_000_000, text_bytes_max: 64 * 1024 * 1024 };

    const fn is_sound(self) -> bool {
        let is_counted = self.node_count_max >= 1;
        let is_roomy = self.text_bytes_max >= self.node_count_max;
        let is_started = TEXT_CAPACITY_INITIAL <= self.text_bytes_max;
        let is_stacked = STACK_CAPACITY_INITIAL <= self.stack_count_max();

        is_counted && is_roomy && is_started && is_stacked
    }

    const fn stack_count_max(self) -> u64 {
        self.node_count_max.strict_mul(4)
    }

    const fn step_count_max(self) -> u64 {
        self.node_count_max.strict_mul(2)
    }

    pub(crate) fn text_bytes_bound(self) -> u64 {
        let bound = self.text_bytes_max.strict_add(size::of_text(TRUNCATION_NOTE));

        assert!(bound > self.text_bytes_max, "the note adds to the budget it reports on");

        bound
    }
}

fn attributes_label(node: &Value) -> Option<&str> {
    let attributes = node.get("attrs")?;

    let label = ATTRIBUTE_KEYS_TEXT.iter().find_map(|key| {
        attributes.get(key).and_then(Value::as_str).filter(|found| !found.is_empty())
    });

    assert!(label.is_none_or(|found| !found.is_empty()), "a label carries visible text");

    label
}

fn blank_lines_collapse(text: &mut String, budget: Budget) {
    let byte_count = size::of_text(text);
    let mut newline_run: u64 = 0;

    assert!(
        byte_count <= budget.text_bytes_bound(),
        "the text arrives inside its budget and its note",
    );

    text.retain(|character| {
        if character == '\n' {
            newline_run = newline_run.strict_add(1);

            return newline_run <= NEWLINE_RUN_MAX;
        }

        newline_run = 0;

        true
    });

    assert!(size::of_text(text) <= byte_count, "collapsing only removes newlines");

    whitespace_trim(text);

    assert!(size::of_text(text) <= byte_count, "trimming only removes whitespace");
}

fn children_push<'document>(walk: &mut Walk<'document>, node: &'document Value) {
    let Some(children) = node.get("content").and_then(Value::as_array) else {
        return;
    };

    let depth = size::of_slice(&walk.stack);
    let stack_count_max = walk.budget.stack_count_max();

    assert!(depth <= stack_count_max, "the stack never outgrows its cap");

    let room = stack_count_max.strict_sub(depth);
    let count = size::of_slice(children).min(room);

    for child in children.iter().take(size::offset(count)).rev() {
        walk.stack.push(Step::Node(child));
    }

    assert!(
        size::of_slice(&walk.stack) <= stack_count_max,
        "pushing the children kept the stack inside its cap",
    );
}

fn date_read(node: &Value) -> Option<String> {
    let timestamp = node.get("attrs")?.get("timestamp")?.as_str()?;
    let timestamp_millis = timestamp.parse::<i64>().ok()?;

    if timestamp_millis.unsigned_abs() > DATE_MILLIS_LIMIT {
        return None;
    }

    let date = date_text(timestamp_millis);

    assert!(date.contains('-'), "a calendar day separates its parts");
    assert!(size::of_text(&date) <= DATE_BYTES_MAX, "a calendar day fits its width");

    Some(date)
}

fn date_text(timestamp_millis: i64) -> String {
    assert!(
        timestamp_millis.unsigned_abs() <= DATE_MILLIS_LIMIT,
        "the timestamp {timestamp_millis} sits outside the range a calendar day is read from",
    );

    let day_count = timestamp_millis.div_euclid(DATE_MILLIS_PER_DAY);
    let day_shifted = day_count.strict_add(DATE_ERA_EPOCH_DAYS);
    let era = day_shifted.div_euclid(DATE_ERA_DAYS);
    let era_day = day_shifted.rem_euclid(DATE_ERA_DAYS);

    let era_year = era_day
        .strict_sub(era_day.div_euclid(1_460))
        .strict_add(era_day.div_euclid(36_524))
        .strict_sub(era_day.div_euclid(146_096))
        .div_euclid(365);

    let year_day = era_day
        .strict_sub(era_year.strict_mul(365))
        .strict_sub(era_year.div_euclid(4))
        .strict_add(era_year.div_euclid(100));

    let month_index = year_day.strict_mul(5).strict_add(2).div_euclid(153);
    let month_start = month_index.strict_mul(153).strict_add(2).div_euclid(5);
    let day = year_day.strict_sub(month_start).strict_add(1);
    let month = month_index.strict_add(if month_index < 10 { 3 } else { -9 });
    let year = era.strict_mul(400).strict_add(era_year).strict_add(i64::from(month <= 2));

    assert!((1..=31).contains(&day), "a day of the month is between 1 and 31");
    assert!((1..=12).contains(&month), "a month is between 1 and 12");

    format!("{year:04}-{month:02}-{day:02}")
}

fn flatten(document: &Value, budget: Budget) -> String {
    assert!(budget.is_sound(), "a flattening budget keeps every relation");

    let mut walk = Walk {
        budget,
        is_truncated: false,
        stack: Vec::with_capacity(size::offset(STACK_CAPACITY_INITIAL)),
        text: String::with_capacity(size::offset(TEXT_CAPACITY_INITIAL)),
    };

    walk.stack.push(Step::Node(document));

    let node_count = steps_run(&mut walk);

    if !walk.stack.is_empty() {
        walk.is_truncated = true;
    }

    if walk.is_truncated {
        walk.text.push_str(TRUNCATION_NOTE);
    }

    assert!(size::of_text(&walk.text) <= budget.text_bytes_bound(), "the text fits its budget");
    assert!(node_count <= budget.node_count_max, "the walk visited at most its node budget");

    blank_lines_collapse(&mut walk.text, budget);

    walk.text
}

fn link_href(node: &Value) -> Option<&str> {
    let marks = node.get("marks")?.as_array()?;

    let link = marks
        .iter()
        .take(size::offset(MARK_COUNT_MAX))
        .find(|mark| mark.get("type").and_then(Value::as_str) == Some("link"))?;

    let href = link
        .get("attrs")
        .and_then(|attributes| attributes.get("href"))
        .and_then(Value::as_str)
        .filter(|found| !found.is_empty());

    assert!(href.is_none_or(|found| !found.is_empty()), "a link target is never empty");

    href
}

fn link_push(walk: &mut Walk<'_>, node: &Value, literal: &str) {
    let Some(href) = link_href(node) else {
        return;
    };

    if href == literal {
        return;
    }

    for part in [" (", href, ")"] {
        if size::of_text(&walk.text) >= walk.budget.text_bytes_max {
            walk.is_truncated = true;

            return;
        }

        text_push(walk, part);
    }

    assert!(
        size::of_text(&walk.text) <= walk.budget.text_bytes_max,
        "a link target fits the budget",
    );
}

fn literal_read(literal: &str, budget: Budget) -> String {
    assert!(budget.is_sound(), "a literal is cut against a sound budget");

    let trimmed = literal.trim();
    let cut = text::prefix(trimmed, budget.text_bytes_max);

    if cut.len() == trimmed.len() {
        return cut.to_owned();
    }

    let mut flattened = cut.to_owned();

    flattened.push_str(TRUNCATION_NOTE);

    assert!(
        size::of_text(&flattened) <= budget.text_bytes_bound(),
        "a cut literal fits its budget",
    );

    assert!(flattened.ends_with(TRUNCATION_NOTE), "a cut literal says it was cut");

    flattened
}

fn step_apply<'document>(walk: &mut Walk<'document>, step: Step<'document>) {
    assert!(
        size::of_slice(&walk.stack) <= walk.budget.stack_count_max(),
        "the stack is inside its cap",
    );

    assert!(
        size::of_text(&walk.text) < walk.budget.text_bytes_max,
        "a step runs only with room left",
    );

    let Step::Node(node) = step else {
        walk.text.push('\n');

        return;
    };

    let node_type = node.get("type").and_then(Value::as_str).unwrap_or("");

    if node_type == "text" {
        if let Some(literal) = node.get("text").and_then(Value::as_str) {
            text_push(walk, literal);
            link_push(walk, node, literal);
        }

        return;
    }

    if node_type == "hardBreak" {
        walk.text.push('\n');

        return;
    }

    if node_type == "date" {
        if let Some(date) = date_read(node) {
            text_push(walk, &date);
        }

        return;
    }

    if let Some(label) = attributes_label(node) {
        if TYPES_TITLED.contains(&node_type) {
            title_push(walk, label);
        } else {
            text_push(walk, label);
        }
    }

    if TYPES_BLOCK.contains(&node_type) {
        walk.stack.push(Step::Break);
    }

    children_push(walk, node);
}

fn steps_run(walk: &mut Walk<'_>) -> u64 {
    let mut node_count: u64 = 0;

    for _step_index in 0..walk.budget.step_count_max() {
        if size::of_text(&walk.text) >= walk.budget.text_bytes_max {
            break;
        }

        if node_count >= walk.budget.node_count_max {
            break;
        }

        let Some(step) = walk.stack.pop() else {
            break;
        };

        if matches!(step, Step::Node(_)) {
            node_count = node_count.strict_add(1);
        }

        step_apply(walk, step);
    }

    assert!(node_count <= walk.budget.node_count_max, "the walk stopped at its node budget");

    node_count
}

fn text_push(walk: &mut Walk<'_>, literal: &str) {
    let used = size::of_text(&walk.text);
    let text_bytes_max = walk.budget.text_bytes_max;

    assert!(used < text_bytes_max, "a push runs only with room left");

    let room = text_bytes_max.strict_sub(used);

    if size::of_text(literal) <= room {
        walk.text.push_str(literal);

        return;
    }

    let boundary = size::offset(text::boundary_floor(literal, room));

    walk.text.push_str(literal.get(..boundary).unwrap_or_default());
    walk.is_truncated = true;

    assert!(size::of_text(&walk.text) <= text_bytes_max, "a cut push fills the budget exactly");
}

#[must_use]
pub fn text_read(field: &Value) -> String {
    text_read_within(field, Budget::GUARD)
}

#[must_use]
pub fn text_read_within(field: &Value, budget: Budget) -> String {
    assert!(budget.is_sound(), "a read is bounded by a sound budget");

    let text = field
        .as_str()
        .map_or_else(|| flatten(field, budget), |literal| literal_read(literal, budget));

    assert!(
        size::of_text(&text) <= budget.text_bytes_bound(),
        "a read text fits its budget and note",
    );

    assert!(!text.starts_with(char::is_whitespace), "a read text opens with its prose");

    text
}

fn title_push(walk: &mut Walk<'_>, title: &str) {
    assert!(!title.is_empty(), "a title carries visible text");

    let mut line = String::with_capacity(title.len().strict_add(1));

    line.push_str(title);
    line.push('\n');

    assert!(line.ends_with('\n'), "a title holds a line of its own");

    text_push(walk, &line);

    assert!(
        size::of_text(&walk.text) <= walk.budget.text_bytes_max,
        "a title fits the budget or is cut to it",
    );
}

fn whitespace_trim(text: &mut String) {
    let end = text.trim_end().len();

    text.truncate(end);

    let begin = text.len().strict_sub(text.trim_start().len());

    text.replace_range(..begin, "");

    assert!(!text.starts_with(char::is_whitespace), "no leading whitespace remains");
    assert!(!text.ends_with(char::is_whitespace), "no trailing whitespace remains");
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::{
        Budget,
        DATE_MILLIS_LIMIT,
        DATE_MILLIS_PER_DAY,
        TRUNCATION_NOTE,
        date_text,
        flatten,
        text_read,
        text_read_within,
    };
    use crate::size;

    const PROBE: Budget = Budget { node_count_max: 20_000, text_bytes_max: 256 * 1024 };

    const DATE_ANCHORS: [(i64, &str); 12] = [
        (0, "1970-01-01"),
        (-1, "1969-12-31"),
        (86_399_999, "1970-01-01"),
        (86_400_000, "1970-01-02"),
        (951_782_400_000, "2000-02-29"),
        (951_868_799_999, "2000-02-29"),
        (951_868_800_000, "2000-03-01"),
        (1_700_000_000_000, "2023-11-14"),
        (4_107_542_400_000, "2100-03-01"),
        (-2_203_891_200_000, "1900-03-01"),
        (253_402_300_799_999, "9999-12-31"),
        (-62_135_596_800_000, "0001-01-01"),
    ];

    struct Link<'link> {
        href: &'link str,
        text: &'link str,
    }

    #[test]
    fn a_budget_is_sound_only_when_every_relation_holds() {
        let sound = Budget { node_count_max: 100, text_bytes_max: 2_048 };

        assert!(sound.is_sound(), "each relation holds");

        for (broken, relation) in [
            (Budget { node_count_max: 0, text_bytes_max: 2_048 }, "no node at all"),
            (Budget { node_count_max: 2_000, text_bytes_max: 1_024 }, "more nodes than bytes"),
            (Budget { node_count_max: 100, text_bytes_max: 512 }, "a text under its first buffer"),
            (Budget { node_count_max: 10, text_bytes_max: 2_048 }, "a stack under its first"),
        ] {
            assert!(!broken.is_sound(), "{relation} is unsound");
        }
    }

    #[test]
    fn a_date_at_the_edge_of_its_range_reads_and_one_millisecond_past_it_does_not() {
        let edge = DATE_MILLIS_LIMIT.to_string();

        assert_eq!(flatten(&date_node(&json!(edge)), Budget::GUARD), "275760-09-13", "the edge");
    }

    #[test]
    fn a_date_outside_the_calendar_reads_as_nothing() {
        let beyond = DATE_MILLIS_LIMIT.strict_add(1).to_string();

        for timestamp in [
            json!("soon"),
            json!(""),
            json!("99999999999999999999"),
            json!(beyond),
            json!(format!("-{beyond}")),
            json!(1_700_000_000_000_i64),
            json!(null),
        ] {
            assert_eq!(
                flatten(&date_node(&timestamp), Budget::GUARD),
                "",
                "the timestamp {timestamp} names no day",
            );
        }

        assert_eq!(flatten(&json!({"type": "date"}), Budget::GUARD), "", "a date with no attrs");
    }

    #[test]
    fn a_date_reaches_both_ends_of_its_range() {
        let limit = i64::try_from(DATE_MILLIS_LIMIT).expect("the limit fits a timestamp");

        assert_eq!(date_text(limit), "275760-09-13", "the last day of the range");
        assert_eq!(date_text(limit.strict_neg()), "-271821-04-20", "the first day of the range");
    }

    #[test]
    fn a_date_reads_as_its_calendar_day_in_utc() {
        for (timestamp_millis, date) in DATE_ANCHORS {
            assert_eq!(date_text(timestamp_millis), date, "the anchor {timestamp_millis}");
        }

        let document = json!({
            "type": "doc",
            "content": [{
                "type": "paragraph",
                "content": [
                    {"type": "text", "text": "due "},
                    date_node(&json!("1700000000000")),
                    {"type": "text", "text": " sharp"},
                ],
            }],
        });

        assert_eq!(
            flatten(&document, Budget::GUARD),
            "due 2023-11-14 sharp",
            "a date reads inline",
        );
    }

    #[test]
    fn a_deeply_nested_document_reaches_the_bottom_without_recursing() {
        let mut document = json!({"type": "text", "text": "bottom"});

        for _depth in 0..120_u32 {
            document = json!({"type": "paragraph", "content": [document]});
        }

        let text = flatten(&document, Budget::GUARD);

        assert_eq!(text, "bottom", "the deepest text survives");
        assert!(!text.contains("truncated"), "a deep document is not a cut one");
    }

    #[test]
    fn a_document_flattens_to_its_prose() {
        let document = json!({
            "type": "doc",
            "version": 1_i64,
            "content": [paragraph("The first paragraph"), paragraph("Second paragraph")],
        });

        assert_eq!(
            text_read(&document),
            "The first paragraph\nSecond paragraph",
            "each paragraph ends a line",
        );
    }

    #[test]
    fn a_document_over_the_node_budget_is_reported_as_truncated() {
        let mut children = Vec::new();

        for index in 0..25_000_u32 {
            children.push(paragraph(&format!("line {index}")));
        }

        let text = flatten(&json!({"type": "doc", "content": children}), PROBE);

        assert!(text.starts_with("line 0"), "the start of the document survives");
        assert!(text.contains("truncated"), "the cut says so");
        assert!(!text.contains("line 24999"), "the end past the budget is dropped");
    }

    #[test]
    fn a_file_in_a_group_keeps_a_line_of_its_own() {
        let document = json!({
            "type": "doc",
            "content": [
                {
                    "type": "mediaGroup",
                    "content": [
                        {"type": "media", "attrs": {"alt": "first.png"}},
                        {"type": "media", "attrs": {"alt": "second.png"}},
                    ],
                },
                {
                    "type": "mediaSingle",
                    "content": [
                        {"type": "media", "attrs": {"alt": "third.png"}},
                        {"type": "caption", "content": [{"type": "text", "text": "the caption"}]},
                    ],
                },
            ],
        });

        let text = flatten(&document, Budget::GUARD);

        for line in ["first.png", "second.png", "third.png", "the caption"] {
            assert!(text.lines().any(|found| found == line), "{line} lost its own line in {text}");
        }
    }

    #[test]
    fn a_hard_break_ends_the_line_without_ending_the_paragraph() {
        let document = json!({
            "type": "doc",
            "content": [{
                "type": "paragraph",
                "content": [
                    {"type": "text", "text": "one"},
                    {"type": "hardBreak"},
                    {"type": "text", "text": "two"},
                ],
            }],
        });

        assert_eq!(flatten(&document, Budget::GUARD), "one\ntwo", "a hard break is one newline");
    }

    #[test]
    fn a_link_at_the_byte_budget_is_cut_and_reported_rather_than_overrun() {
        let document = json!({
            "type": "doc",
            "content": [{
                "type": "paragraph",
                "content": [{
                    "type": "text",
                    "text": repeated("a", PROBE.text_bytes_max.strict_sub(1)),
                    "marks": [{"type": "link", "attrs": {"href": "https://example.com"}}],
                }],
            }],
        });

        let text = flatten(&document, PROBE);

        assert!(text.ends_with(TRUNCATION_NOTE), "a cut document says so");

        assert!(
            size::of_text(&text) <= PROBE.text_bytes_max.strict_add(size::of_text(TRUNCATION_NOTE)),
            "the cut text fits its budget and its note",
        );

        assert!(!text.contains("example.com"), "the target that did not fit is dropped");
    }

    #[test]
    fn a_link_keeps_its_target_beside_its_text() {
        let outline = linked(&Link { href: "https://example.com/outline", text: "the outline" });
        let bare = "https://example.com/raw";
        let raw = linked(&Link { href: bare, text: bare });

        let document = json!({
            "type": "doc",
            "content": [{
                "type": "paragraph",
                "content": [
                    {"type": "text", "text": "see "},
                    outline,
                    {"type": "text", "text": " and "},
                    raw,
                ],
            }],
        });

        assert_eq!(
            flatten(&document, Budget::GUARD),
            "see the outline (https://example.com/outline) and https://example.com/raw",
            "a link shows its target unless the text already is the target",
        );
    }

    #[test]
    fn a_plain_string_field_is_cut_at_the_byte_budget() {
        let bound = PROBE.text_bytes_max.strict_add(64);

        let text =
            text_read_within(&json!(repeated("a", PROBE.text_bytes_max.strict_mul(2))), PROBE);

        assert!(size::of_text(&text) <= bound, "the cut string fits its budget");
        assert!(text.starts_with("aaa"), "the cut keeps the start");
        assert!(text.contains("truncated"), "the cut says so");

        let wide_text = text_read_within(
            &json!(repeated("\u{1f600}", PROBE.text_bytes_max.strict_mul(2))),
            PROBE,
        );

        assert!(size::of_text(&wide_text) <= bound, "a wide cut fits its budget");
        assert!(wide_text.starts_with('\u{1f600}'), "a wide cut keeps whole characters");
        assert!(wide_text.contains("truncated"), "a wide cut says so");
    }

    #[test]
    fn a_plain_string_field_passes_straight_through() {
        assert_eq!(text_read(&json!("just a string")), "just a string", "a string reads as is");
        assert_eq!(text_read(&json!("  padded  ")), "padded", "a string is trimmed");
        assert_eq!(text_read(&json!(null)), "", "a missing field reads as nothing");
    }

    #[test]
    fn a_stack_past_its_cap_is_reported_as_truncated() {
        let budget = Budget { node_count_max: 100, text_bytes_max: 2_048 };
        let children = vec![paragraph("x"); 1_000];
        let text = flatten(&json!({"type": "doc", "content": children}), budget);

        assert!(text.ends_with(TRUNCATION_NOTE), "children past the stack cap are never silent");
    }

    #[test]
    fn a_text_of_the_largest_size_atlassian_allows_flattens_whole_under_the_guard() {
        let largest = 25 * 1024 * 1024;
        let document = json!({"type": "doc", "content": [paragraph(&repeated("a", largest))]});
        let text = text_read(&document);

        assert_eq!(size::of_text(&text), largest, "every byte Atlassian allows survives");
        assert!(!text.ends_with(TRUNCATION_NOTE), "an allowed text is never reported as cut");
    }

    #[test]
    fn a_title_at_the_byte_budget_is_cut_and_reported_rather_than_overrun() {
        for shortfall in [0, 1, 2] {
            let title = repeated("a", PROBE.text_bytes_max.strict_sub(shortfall));

            let document = json!({
                "type": "expand",
                "attrs": {"title": title},
                "content": [paragraph("inside")],
            });

            let text = flatten(&document, PROBE);

            assert!(text.ends_with(TRUNCATION_NOTE), "a cut document says so");
            assert!(!text.contains("inside"), "the body past the budget is dropped");

            assert!(
                size::of_text(&text) <= PROBE.text_bytes_bound(),
                "the cut text fits its budget and its note",
            );
        }
    }

    #[test]
    fn an_empty_or_unknown_document_flattens_to_nothing() {
        assert_eq!(
            flatten(&json!({"type": "doc", "content": []}), Budget::GUARD),
            "",
            "an empty doc",
        );

        assert_eq!(flatten(&json!({}), Budget::GUARD), "", "an empty object");
        assert_eq!(flatten(&json!(null), Budget::GUARD), "", "a null");
        assert_eq!(flatten(&json!({"type": "somethingNew"}), Budget::GUARD), "", "an unknown node");
    }

    #[test]
    fn an_expand_keeps_its_title_on_a_line_above_its_content() {
        for kind in ["expand", "nestedExpand"] {
            let expand = json!({
                "type": kind,
                "attrs": {"title": "The details"},
                "content": [paragraph("inside")],
            });

            let document = json!({"type": "doc", "content": [expand, paragraph("after")]});

            assert_eq!(
                flatten(&document, Budget::GUARD),
                "The details\ninside\nafter",
                "the {kind} title is a line of its own",
            );
        }

        let untitled =
            json!({"type": "expand", "attrs": {"title": ""}, "content": [paragraph("x")]});

        assert_eq!(flatten(&untitled, Budget::GUARD), "x", "an untitled expand adds no line");
    }

    fn calendar_day_next(day: [i64; 3]) -> [i64; 3] {
        assert!(day.iter().skip(1).all(|part| *part >= 1), "{day:?} counts its month from 1");

        let [year, month, day_of_month] = day;
        let is_century = year.rem_euclid(100) == 0;
        let is_leap = year.rem_euclid(if is_century { 400 } else { 4 }) == 0;
        let february = if is_leap { 29 } else { 28 };
        let lengths = [31, february, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let length = lengths[usize::try_from(month.strict_sub(1)).expect("a month is positive")];

        assert!(day_of_month <= length, "the day {day:?} fits its month");

        if day_of_month < length {
            return [year, month, day_of_month.strict_add(1)];
        }

        if month < 12 {
            return [year, month.strict_add(1), 1];
        }

        [year.strict_add(1), 1, 1]
    }

    fn calendar_day_parse(text: &str) -> [i64; 3] {
        let sign: i64 = if text.starts_with('-') { -1 } else { 1 };
        let digits = text.trim_start_matches('-');
        let parts: Vec<i64> = digits.split('-').flat_map(str::parse).collect();

        assert_eq!(parts.len(), 3, "{text} has a year, a month, and a day");
        assert!((1..=12).contains(&parts[1]), "{text} names a month");

        [sign.strict_mul(parts[0]), parts[1], parts[2]]
    }

    #[test]
    fn consecutive_days_walk_the_gregorian_calendar() {
        let day_first: i64 = -1_100_000;
        let day_count: i64 = 2_200_000;
        let millis_first = day_first.strict_mul(DATE_MILLIS_PER_DAY);
        let mut previous = calendar_day_parse(&date_text(millis_first));

        for day in 1..=day_count {
            let millis = day_first.strict_add(day).strict_mul(DATE_MILLIS_PER_DAY);
            let current = calendar_day_parse(&date_text(millis));

            assert_eq!(current, calendar_day_next(previous), "the day at {millis} ms");

            previous = current;
        }

        assert!(previous[0] >= 2100, "the walk crosses a century that is not a leap year");
    }

    fn date_node(timestamp: &Value) -> Value {
        assert!(!timestamp.is_object(), "a timestamp is a scalar, {timestamp}");

        json!({"type": "date", "attrs": {"timestamp": timestamp}})
    }

    fn decision_item(text: &str) -> Value {
        assert!(!text.is_empty(), "a decision says something");

        json!({
            "type": "decisionItem",
            "attrs": {"localId": "4a2d86d0", "state": "DECIDED"},
            "content": [{"type": "text", "text": text}],
        })
    }

    #[test]
    fn every_task_decision_and_card_keeps_a_line_of_its_own() {
        let document = json!({
            "type": "doc",
            "content": [
                {
                    "type": "taskList",
                    "content": [task_item("First task"), task_item("Second task")],
                },
                {"type": "decisionList", "content": [decision_item("Ship it")]},
                {"type": "blockCard", "attrs": {"url": "https://example.com/card"}},
                paragraph("after"),
            ],
        });

        let text = flatten(&document, Budget::GUARD);

        for line in ["First task", "Second task", "Ship it", "https://example.com/card", "after"] {
            assert!(text.lines().any(|found| found == line), "{line} lost its own line in {text}");
        }
    }

    #[test]
    fn inline_nodes_keep_their_visible_text() {
        let document = json!({
            "type": "doc",
            "content": [{
                "type": "paragraph",
                "content": [
                    {"type": "text", "text": "ping "},
                    {"type": "mention", "attrs": {"id": "abc", "text": "@Jane Doe"}},
                    {"type": "text", "text": " about "},
                    {"type": "inlineCard", "attrs": {"url": "https://example.com/x"}},
                ],
            }],
        });

        assert_eq!(
            flatten(&document, Budget::GUARD),
            "ping @Jane Doe about https://example.com/x",
            "a mention and a card keep the text they show",
        );
    }

    fn linked(link: &Link<'_>) -> Value {
        assert!(!link.href.is_empty(), "a link has a target");

        json!({
            "type": "text",
            "text": link.text,
            "marks": [{"type": "strong"}, {"type": "link", "attrs": {"href": link.href}}],
        })
    }

    #[test]
    fn nesting_does_not_pile_up_blank_lines() {
        let document = json!({
            "type": "doc",
            "content": [{
                "type": "panel",
                "content": [{
                    "type": "bulletList",
                    "content": [
                        {"type": "listItem", "content": [paragraph("first")]},
                        {"type": "listItem", "content": [paragraph("second")]},
                    ],
                }],
            }],
        });

        let text = flatten(&document, Budget::GUARD);

        assert!(text.contains("first"), "the first item survives");
        assert!(text.contains("second"), "the second item survives");
        assert!(!text.contains("\n\n\n"), "no more than one blank line in a row");
    }

    #[test]
    fn one_enormous_text_node_is_cut_at_the_byte_budget() {
        let bound = PROBE.text_bytes_max.strict_add(64);

        let document = json!({
            "type": "doc",
            "content": [paragraph(&repeated("a", PROBE.text_bytes_max.strict_mul(2)))],
        });

        let text = flatten(&document, PROBE);

        assert!(size::of_text(&text) <= bound, "the cut text fits its budget");
        assert!(text.starts_with("aaa"), "the cut keeps the start");
        assert!(text.contains("truncated"), "the cut says so");

        let wide = json!({
            "type": "doc",
            "content": [paragraph(&repeated("\u{1f600}", PROBE.text_bytes_max.strict_mul(2)))],
        });

        let wide_text = flatten(&wide, PROBE);

        assert!(size::of_text(&wide_text) <= bound, "a wide cut fits its budget");
        assert!(wide_text.starts_with('\u{1f600}'), "a wide cut keeps whole characters");
        assert!(wide_text.contains("truncated"), "a wide cut says so");
    }

    fn paragraph(text: &str) -> Value {
        json!({"type": "paragraph", "content": [{"type": "text", "text": text}]})
    }

    fn repeated(piece: &str, bytes: u64) -> String {
        assert!(!piece.is_empty(), "a piece repeats into something");
        assert!(bytes >= size::of_text(piece), "a repetition holds at least one piece");

        piece.repeat(size::offset(bytes.div_euclid(size::of_text(piece))))
    }

    fn task_item(text: &str) -> Value {
        assert!(!text.is_empty(), "a task says something");

        json!({
            "type": "taskItem",
            "attrs": {"localId": "4a2d86d0", "state": "TODO"},
            "content": [{"type": "text", "text": text}],
        })
    }
}
