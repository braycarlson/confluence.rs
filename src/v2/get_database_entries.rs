#[derive(Clone, Copy, Debug)]
pub struct GetDatabaseEntries<'call> {
    pub id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub cursor: Option<&'options str>,
    pub filter: Option<&'options [(&'options str, &'options str)]>,
    pub if_none_match: Option<&'options str>,
    pub limit: Option<i64>,
    pub sort: Option<&'options str>,
}

impl crate::Operation for GetDatabaseEntries<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/databases/{id}/entries";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::DatabaseEntryListResponse>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("cursor", options.cursor);
        request.query_pairs("filter", options.filter)?;
        request.path_text("id", self.id)?;
        request.header_text("If-None-Match", options.if_none_match)?;
        request.query_number("limit", options.limit);
        request.query_text("sort", options.sort);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetDatabaseEntries<'_> {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "cursor",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::DatabaseEntry>;
}
