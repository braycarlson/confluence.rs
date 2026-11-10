#[derive(Clone, Copy, Debug)]
pub struct SearchUser<'call> {
    pub cql: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options [&'options str]>,
    pub limit: Option<i64>,
    pub site_permission_type_filter: Option<&'options str>,
    pub start: Option<i64>,
}

impl crate::Operation for SearchUser<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/search/user";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::SearchPageResponseSearchResult>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("cql", Some(self.cql));
        request.query_texts_joined("expand", options.expand)?;
        request.query_number("limit", options.limit);
        request.query_text("sitePermissionTypeFilter", options.site_permission_type_filter);
        request.query_number("start", options.start);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for SearchUser<'_> {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "start",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::SearchResult>;
}
