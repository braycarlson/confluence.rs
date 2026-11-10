use core::marker::PhantomData;

use serde_json::Value;

use crate::encoding;
use crate::error::Error;
use crate::items::Items;
use crate::kind::Kind;
use crate::limits;
use crate::paging::{Next, Paging};
use crate::size;

const FIELD_TOTAL: &str = "totalSize";

#[derive(Clone, Debug)]
pub struct Page<Item> {
    count: u64,
    item: PhantomData<Item>,
    items: &'static str,
    token: Option<String>,
    total: Option<u64>,
    value: Value,
}

impl<Item> Page<Item>
where
    Item: Kind,
{
    #[must_use]
    pub fn count(&self) -> u64 {
        assert!(self.is_counted(), "a page holds the items it counted");

        self.count
    }

    #[must_use]
    pub fn count_total(&self) -> Option<u64> {
        assert!(self.is_counted(), "a page holds the items it counted");

        self.total
    }

    #[must_use]
    pub fn into_values(mut self) -> Vec<Value> {
        assert!(self.is_counted(), "a page holds the items it counted");

        let held = self.value.pointer_mut(self.items).map(Value::take);

        let values = match held {
            Some(Value::Array(values)) => values,
            Some(_) | None => Vec::new(),
        };

        assert_eq!(size::of_slice(&values), self.count, "a page hands over the items it counted");

        values
    }

    fn is_counted(&self) -> bool {
        assert!(self.items.starts_with('/'), "an array is named by a pointer");

        let found = items_read(&self.value, self.items);

        found.is_some_and(|items| size::of_slice(items) == self.count)
    }

    #[must_use]
    pub fn is_last(&self) -> bool {
        assert!(self.token.as_deref().is_none_or(|held| !held.is_empty()), "a token is text");

        self.token.is_none()
    }

    #[must_use]
    pub fn items(&self) -> Items<'_, Item> {
        let items = items_read(&self.value, self.items).unwrap_or_default();

        assert_eq!(size::of_slice(items), self.count, "a page holds the items it counted");

        Items::of(items)
    }

    pub(crate) fn parse(value: Value, paging: Paging) -> Result<Self, Error> {
        assert!(!paging.cursor.is_empty(), "a walk names its cursor");

        let Some(found) = items_read(&value, paging.items) else {
            return Err(Error::Reply(format!("page missing array field {}", paging.items)));
        };

        let count = size::of_slice(found);
        let total = total_read(&value);

        link_check(&value, paging)?;

        let token = match token_find(&value, paging) {
            Some(text) => Some(token_check(&text)?),
            None => None,
        };

        let page = Self { count, item: PhantomData, items: paging.items, token, total, value };

        assert!(page.is_counted(), "a page holds the items it counted");

        Ok(page)
    }

    #[must_use]
    pub(crate) fn token_next(&self) -> Option<&str> {
        let token = self.token.as_deref();

        if let Some(held) = token {
            assert!(!held.is_empty(), "a page token is never empty");

            assert!(
                size::of_text(held) <= u64::from(limits::TOKEN_BYTES_MAX),
                "a page token fits its bound",
            );
        }

        token
    }

    #[must_use]
    pub fn value(&self) -> &Value {
        assert!(self.is_counted(), "a page holds the items it counted");

        &self.value
    }
}

fn items_read<'value>(value: &'value Value, items: &str) -> Option<&'value [Value]> {
    assert!(items.starts_with('/'), "an array is named by a pointer");
    assert!(items.len() >= 2, "a pointer names a member");

    value.pointer(items)?.as_array().map(Vec::as_slice)
}

fn link_check(value: &Value, paging: Paging) -> Result<(), Error> {
    assert!(!paging.cursor.is_empty(), "a walk names its cursor");

    let Next::Link(pointer) = paging.next else {
        return Ok(());
    };

    let link = value.pointer(pointer).and_then(Value::as_str).unwrap_or_default();

    if link.is_empty() {
        return Ok(());
    }

    if link_cursor(link, paging).is_none() {
        return Err(Error::Reply(format!("the next link names no {} term", paging.cursor)));
    }

    assert!(!link.is_empty(), "a checked link names somewhere");

    Ok(())
}

fn link_cursor(link: &str, paging: Paging) -> Option<String> {
    assert!(!paging.cursor.is_empty(), "a walk names its cursor");
    assert!(paging.cursor.is_ascii(), "a cursor parameter is a plain name");

    let (_address, query) = link.split_once('?')?;
    let prefix = format!("{}=", paging.cursor);
    let term = query.split('&').find_map(|found| found.strip_prefix(prefix.as_str()))?;
    let decoded = encoding::percent_decode(term);

    String::from_utf8(decoded).ok().filter(|text| !text.is_empty())
}

fn token_check(token: &str) -> Result<String, Error> {
    if size::of_text(token) > u64::from(limits::TOKEN_BYTES_MAX) {
        return Err(Error::Reply(format!(
            "page carried a next token over {} bytes",
            limits::TOKEN_BYTES_MAX,
        )));
    }

    assert!(!token.is_empty(), "a kept token is never empty");

    Ok(token.to_owned())
}

fn token_find(value: &Value, paging: Paging) -> Option<String> {
    assert!(!paging.cursor.is_empty(), "a walk names its cursor");
    assert!(paging.cursor.is_ascii(), "a cursor parameter is a plain name");

    match paging.next {
        Next::Link(pointer) => {
            let link = value.pointer(pointer).and_then(Value::as_str)?;

            link_cursor(link, paging)
        }
        Next::Token(pointer) => {
            let token = value.pointer(pointer).and_then(Value::as_str)?;

            Some(token.to_owned()).filter(|text| !text.is_empty())
        }
    }
}

fn total_read(value: &Value) -> Option<u64> {
    assert!(value.is_object(), "a reply that holds a named array is an object");
    assert!(!FIELD_TOTAL.is_empty(), "a total has a name");

    value.get(FIELD_TOTAL).and_then(Value::as_u64)
}

#[cfg(test)]
mod tests {
    use core::marker::PhantomData;

    use serde_json::{Value, json};

    use super::{Page, link_cursor, token_check};
    use crate::error::Error;
    use crate::kind::{Any, Text};
    use crate::limits;
    use crate::paging::{Next, Paging};
    use crate::size;

    const LINK: Paging =
        Paging { cursor: "cursor", items: "/results", next: Next::Link("/_links/next") };

    const START: Paging =
        Paging { cursor: "start", items: "/results", next: Next::Link("/_links/next") };

    const TOKEN: Paging =
        Paging { cursor: "cursor", items: "/results", next: Next::Token("/cursor") };

    #[test]
    fn a_link_names_its_cursor_among_its_query_terms() {
        let link = "/wiki/api/v2/pages?limit=5&cursor=a%2Fb";
        let other = Paging { cursor: "after", items: "/results", next: Next::Link("/next") };

        assert_eq!(link_cursor(link, LINK).as_deref(), Some("a/b"), "decoded");
        assert_eq!(link_cursor(link, other), None, "a term it does not hold");
        assert_eq!(link_cursor("/wiki/api/v2/pages", LINK), None, "no query at all");
        assert_eq!(link_cursor("", LINK), None, "a blank link ends the walk rather than a run");
    }

    #[test]
    fn a_link_page_ends_where_the_link_does() {
        let middle = page(json!({"_links": {"next": "/x?cursor=t"}, "results": ["a"]}), LINK);
        let last = page(json!({"_links": {"base": "https://x"}, "results": ["a"]}), LINK);
        let blank = page(json!({"_links": {"next": ""}, "results": ["a"]}), LINK);

        assert!(!middle.is_last(), "a link leads on");
        assert_eq!(middle.token_next(), Some("t"), "the cursor term is kept");
        assert!(last.is_last(), "no next link ends the walk");
        assert!(blank.is_last(), "a blank next link ends it too");
    }

    #[test]
    fn a_page_hands_over_every_item_wherever_the_reply_holds_them() {
        let items = json!([{"id": 1_i64}, {"id": 2_i64}]);
        let data = Paging { cursor: "cursor", items: "/data", next: Next::Link("/links/next") };

        for (body, paging) in [
            (json!({"results": items, "_links": {}}), LINK),
            (json!({"data": items, "links": {}}), data),
        ] {
            let page: Page<Any> = Page::parse(body, paging).expect("a page");
            let count = page.items().count();
            let values = page.into_values();

            assert_eq!(Some(&values), items.as_array(), "the items arrive whole and in order");
            assert_eq!(values.len(), count, "owning the items loses none of them");
        }

        let empty: Page<Any> = Page::parse(json!({"results": []}), LINK).expect("an empty page");

        assert!(empty.into_values().is_empty(), "an empty page hands over nothing");
    }

    #[test]
    fn a_page_reads_its_items_and_keeps_its_value() {
        let parsed = page(json!({"totalSize": 3_i64, "results": ["a", "b", "c"]}), START);
        let items: Vec<&str> = parsed.items().collect();

        assert_eq!(items, ["a", "b", "c"], "every item");
        assert_eq!(parsed.count(), 3, "the count");
        assert_eq!(parsed.count_total(), Some(3), "the reported total");
        assert_eq!(parsed.value()["totalSize"], 3_i64, "the raw reply");
        assert_eq!(page(json!({"results": []}), START).count_total(), None, "no total reported");
    }

    #[test]
    fn a_page_refuses_what_it_cannot_walk() {
        let long = "t".repeat(4_096);
        let unlinked = json!({"_links": {"next": "/x?other=1"}, "results": ["a"]});

        let refused = [
            Page::<Text>::parse(Value::Null, LINK),
            Page::<Text>::parse(json!(["a"]), LINK),
            Page::<Text>::parse(json!({"other": []}), TOKEN),
            Page::<Text>::parse(json!({"results": {}}), TOKEN),
            Page::<Text>::parse(json!({"cursor": long, "results": []}), TOKEN),
            Page::<Text>::parse(unlinked, LINK),
        ];

        for parsed in refused {
            assert!(matches!(parsed, Err(Error::Reply(_))), "the reply cannot be walked");
        }
    }

    #[test]
    fn a_page_whose_count_disagrees_with_its_items_is_caught() {
        let held = page(json!({"results": ["a"]}), TOKEN);

        let broken: Page<Text> = Page {
            count: 2,
            item: PhantomData,
            items: "/results",
            token: None,
            total: None,
            value: json!({"results": ["a"]}),
        };

        assert!(!broken.is_counted(), "a count the items disagree with");
        assert!(held.is_counted(), "a parsed page always agrees with itself");
    }

    #[test]
    fn a_relative_link_with_an_offset_names_its_start() {
        let link = "/rest/api/group?next=true&limit=2&start=2";
        let parsed = page(json!({"_links": {"next": link}, "results": ["a", "b"]}), START);

        assert!(!parsed.is_last(), "the link leads on");
        assert_eq!(parsed.token_next(), Some("2"), "the start term is the cursor");
    }

    #[test]
    fn a_token_at_its_limit_is_kept_and_one_byte_more_is_refused() {
        let limit = size::offset(u64::from(limits::TOKEN_BYTES_MAX));

        assert!(token_check(&"t".repeat(limit)).is_ok(), "a token at the limit");
        assert!(token_check(&"t".repeat(limit.strict_add(1))).is_err(), "a token past it");
    }

    #[test]
    fn a_token_decides_a_token_page_even_when_the_page_is_empty() {
        let filtered = page(json!({"cursor": "t2", "results": []}), TOKEN);

        assert!(!filtered.is_last(), "an empty page with a token still leads on");
        assert_eq!(filtered.count(), 0, "with nothing on it");
        assert_eq!(filtered.token_next(), Some("t2"), "and the token kept");
    }

    #[test]
    fn a_token_page_ends_where_the_token_does() {
        let middle = page(json!({"cursor": "t", "results": ["a"]}), TOKEN);
        let last = page(json!({"results": ["a"]}), TOKEN);
        let blank = page(json!({"cursor": "", "results": ["a"]}), TOKEN);
        let null = page(json!({"cursor": null, "results": ["a"]}), TOKEN);

        assert!(!middle.is_last(), "a token leads on");
        assert_eq!(middle.token_next(), Some("t"), "the token is kept");
        assert!(last.is_last(), "no token ends the walk");
        assert!(blank.is_last(), "an empty token ends it");
        assert!(null.is_last(), "a null token ends it");
    }

    fn page(value: Value, paging: Paging) -> Page<Text> {
        assert!(value.is_object(), "a fixture page is a reply");
        assert!(paging.items.starts_with('/'), "a fixture names its array");

        Page::parse(value, paging).expect("the page parses")
    }
}
