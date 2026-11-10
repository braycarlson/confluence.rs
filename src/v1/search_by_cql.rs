#[derive(Clone, Copy, Debug)]
pub struct SearchByCQL<'call> {
    pub cql: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub cache_buster: Option<i64>,
    pub cqlcontext: Option<&'options str>,
    pub cursor: Option<&'options str>,
    pub excerpt: Option<&'options str>,
    pub exclude_spaces_current: Option<bool>,
    pub expand: Option<&'options [&'options str]>,
    pub include_archived_spaces: Option<bool>,
    pub limit: Option<i64>,
    pub next: Option<bool>,
    pub previous: Option<bool>,
    pub site_permission_type_filter: Option<&'options str>,
    pub start: Option<i64>,
}

impl crate::Operation for SearchByCQL<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/search";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::SearchPageResponseSearchResult>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_number("_", options.cache_buster);
        request.query_text("cql", Some(self.cql));
        request.query_text("cqlcontext", options.cqlcontext);
        request.query_text("cursor", options.cursor);
        request.query_text("excerpt", options.excerpt);
        request.query_boolean("excludeCurrentSpaces", options.exclude_spaces_current);
        request.query_texts_joined("expand", options.expand)?;
        request.query_boolean("includeArchivedSpaces", options.include_archived_spaces);
        request.query_number("limit", options.limit);
        request.query_boolean("next", options.next);
        request.query_boolean("prev", options.previous);
        request.query_text("sitePermissionTypeFilter", options.site_permission_type_filter);
        request.query_number("start", options.start);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for SearchByCQL<'_> {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "cursor",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::SearchResult>;
}
