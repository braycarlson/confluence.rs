#[derive(Clone, Copy, Debug)]
pub struct GetDatabaseViews<'call> {
    pub id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub cursor: Option<&'options str>,
    pub if_none_match: Option<&'options str>,
    pub limit: Option<i64>,
}

impl crate::Operation for GetDatabaseViews<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/databases/{id}/views";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::DatabaseViewListResponse>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("cursor", options.cursor);
        request.path_text("id", self.id)?;
        request.header_text("If-None-Match", options.if_none_match)?;
        request.query_number("limit", options.limit);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetDatabaseViews<'_> {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "cursor",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::DatabaseView>;
}
