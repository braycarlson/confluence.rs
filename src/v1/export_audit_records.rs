#[derive(Clone, Copy, Debug)]
pub struct ExportAuditRecords;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub end_date: Option<&'options str>,
    pub format: Option<&'options str>,
    pub search_string: Option<&'options str>,
    pub start_date: Option<&'options str>,
}

impl crate::Operation for ExportAuditRecords {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/audit/export";

    type Options<'options> = Options<'options>;
    type Reply = crate::Binary;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("endDate", options.end_date);
        request.query_text("format", options.format);
        request.query_text("searchString", options.search_string);
        request.query_text("startDate", options.start_date);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
