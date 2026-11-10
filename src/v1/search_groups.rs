#[derive(Clone, Copy, Debug)]
pub struct SearchGroups<'call> {
    pub query: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub limit: Option<i64>,
    pub should_return_size_total: Option<bool>,
    pub start: Option<i64>,
}

impl crate::Operation for SearchGroups<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/group/picker";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::GroupArrayWithLinks>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_number("limit", options.limit);
        request.query_text("query", Some(self.query));
        request.query_boolean("shouldReturnTotalSize", options.should_return_size_total);
        request.query_number("start", options.start);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for SearchGroups<'_> {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "start",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::Group>;
}
