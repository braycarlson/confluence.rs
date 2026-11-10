#[derive(Clone, Copy, Debug)]
pub struct GetAuditRecordsForTimePeriod;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub limit: Option<i64>,
    pub number: Option<i64>,
    pub search_string: Option<&'options str>,
    pub start: Option<i64>,
    pub units: Option<&'options str>,
}

impl crate::Operation for GetAuditRecordsForTimePeriod {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/audit/since";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::AuditRecordArray>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_number("limit", options.limit);
        request.query_number("number", options.number);
        request.query_text("searchString", options.search_string);
        request.query_number("start", options.start);
        request.query_text("units", options.units);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetAuditRecordsForTimePeriod {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "start",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::AuditRecord>;
}
