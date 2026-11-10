use core::fmt;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::kind::Kind;
use crate::limits;
use crate::page::Page;
use crate::paged::Paged;
use crate::paging::Paging;
use crate::transport::Transport;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Cursor {
    First,
    Stalled,
    Token(String),
}

pub struct Walk<'client, 'call, Network, Walked>
where
    Walked: Paged,
{
    client: &'client Client<Network>,
    cursor: Option<Cursor>,
    operation: &'call Walked,
    options: Walked::Options<'call>,
    page_count_remaining: u32,
}

impl Cursor {
    #[must_use]
    fn after<Item>(&self, page: &Page<Item>, paging: Paging) -> Option<Self>
    where
        Item: Kind,
    {
        assert!(!paging.cursor.is_empty(), "a walk names its cursor");
        assert_ne!(*self, Self::Stalled, "a stalled walk fetches nothing");

        if page.is_last() {
            return None;
        }

        let following = page.token_next().map(|token| Self::Token(token.to_owned()));

        if following.as_ref() == Some(self) {
            return Some(Self::Stalled);
        }

        following
    }

    #[must_use]
    pub(crate) fn text(&self) -> Option<&str> {
        assert_ne!(*self, Self::Stalled, "a stalled walk fetches nothing");

        let text = match self {
            Self::First | Self::Stalled => None,
            Self::Token(token) => Some(token.as_str()),
        };

        assert_eq!(text.is_none(), *self == Self::First, "only the first page names no cursor");
        assert!(text.is_none_or(|found| !found.is_empty()), "a cursor is never empty");

        text
    }
}

impl<'client, 'call, Network, Walked> Walk<'client, 'call, Network, Walked>
where
    Network: Transport,
    Walked: Paged,
{
    #[must_use]
    pub(crate) fn new(
        client: &'client Client<Network>,
        operation: &'call Walked,
        options: Walked::Options<'call>,
    ) -> Self {
        let walk = Self {
            client,
            cursor: Some(Cursor::First),
            operation,
            options,
            page_count_remaining: limits::PAGE_COUNT_MAX,
        };

        assert_eq!(walk.cursor, Some(Cursor::First), "a walk starts at the beginning");
        assert!(walk.page_count_remaining >= 1, "a walk may read at least one page");

        walk
    }

    fn page_next(&mut self, cursor: &Cursor) -> Result<Page<Walked::Item>, Error> {
        assert!(self.page_count_remaining < limits::PAGE_COUNT_MAX, "a page was budgeted");
        assert!(self.cursor.is_none(), "the cursor in hand was taken from the walk");

        let request = self.client.page_request(self.operation, &self.options, cursor)?;
        let value: Value = self.client.page_fetch(&request)?;
        let page = Page::parse(value, Walked::PAGING)?;

        self.cursor = cursor.after(&page, Walked::PAGING);

        Ok(page)
    }
}

impl<Network, Walked> fmt::Debug for Walk<'_, '_, Network, Walked>
where
    Walked: Paged,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Walk")
            .field("cursor", &self.cursor)
            .field("operation", &self.operation)
            .field("page_count_remaining", &self.page_count_remaining)
            .finish_non_exhaustive()
    }
}

impl<Network, Walked> Iterator for Walk<'_, '_, Network, Walked>
where
    Network: Transport,
    Walked: Paged,
{
    type Item = Result<Page<Walked::Item>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let cursor = self.cursor.take()?;

        if cursor == Cursor::Stalled {
            return Some(Err(Error::Reply(format!(
                "a walk of {} was handed the token it just used",
                Walked::PATH,
            ))));
        }

        if self.page_count_remaining == 0 {
            return Some(Err(Error::Reply(format!(
                "a walk of {} ran past its budget of {} pages",
                Walked::PATH,
                limits::PAGE_COUNT_MAX,
            ))));
        }

        self.page_count_remaining = self.page_count_remaining.strict_sub(1);

        let page = self.page_next(&cursor);

        assert!(self.page_count_remaining < limits::PAGE_COUNT_MAX, "a page was spent");

        Some(page)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::Cursor;
    use crate::kind::Text;
    use crate::page::Page;
    use crate::paging::{Next, Paging};

    const LINK: Paging =
        Paging { cursor: "cursor", items: "/data", next: Next::Link("/links/next") };

    const TOKEN: Paging =
        Paging { cursor: "cursor", items: "/results", next: Next::Token("/cursor") };

    #[test]
    fn a_link_cursor_reads_the_token_out_of_the_next_address() {
        let body = json!({"data": ["a"], "links": {"next": "/rules?limit=5&cursor=a%2Fb"}});
        let page = Page::<Text>::parse(body, LINK).expect("a page");

        assert_eq!(
            Cursor::First.after(&page, LINK),
            Some(Cursor::Token("a/b".to_owned())),
            "the decoded token, ready to be escaped again",
        );
    }

    #[test]
    fn a_token_confluence_hands_back_twice_stalls_the_walk() {
        let body = json!({"cursor": "t2", "results": ["a"]});
        let page = Page::<Text>::parse(body, TOKEN).expect("a page");
        let held = Cursor::Token("t2".to_owned());

        assert_eq!(held.after(&page, TOKEN), Some(Cursor::Stalled), "the same token again");
        assert_eq!(page.count(), 1, "the page in hand still counts");
    }

    #[test]
    fn a_token_cursor_follows_the_token_and_a_last_page_ends_the_walk() {
        let body = json!({"cursor": "t2", "results": ["a"]});
        let middle = Page::<Text>::parse(body, TOKEN).expect("a page");
        let last = Page::<Text>::parse(json!({"results": ["a"]}), TOKEN).expect("a page");

        assert_eq!(
            Cursor::First.after(&middle, TOKEN),
            Some(Cursor::Token("t2".to_owned())),
            "the next token",
        );

        assert_eq!(Cursor::First.after(&last, TOKEN), None, "no token ends the walk");
        assert_eq!(Cursor::Token("t1".to_owned()).after(&last, TOKEN), None, "nor a later page");
    }

    #[test]
    fn only_the_first_page_names_no_cursor() {
        assert_eq!(Cursor::First.text(), None, "the first page");
        assert_eq!(Cursor::Token("t".to_owned()).text(), Some("t"), "a token");
    }
}
