#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Next {
    Link(&'static str),
    Token(&'static str),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Paging {
    pub cursor: &'static str,
    pub items: &'static str,
    pub next: Next,
}

#[cfg(test)]
mod tests {
    use super::{Next, Paging};

    #[test]
    fn a_paging_names_its_cursor_its_items_and_how_a_page_continues() {
        let link = Paging { cursor: "cursor", items: "/results", next: Next::Link("/_links/next") };
        let token = Paging { cursor: "cursor", items: "/results", next: Next::Token("/cursor") };

        assert_eq!(link.cursor, "cursor", "the request parameter");
        assert_eq!(link.items, "/results", "a pointer to the array");
        assert_ne!(link.next, token.next, "the two styles differ");
    }
}
