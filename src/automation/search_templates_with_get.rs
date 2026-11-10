#[derive(Clone, Copy, Debug)]
pub struct SearchTemplatesWithGet;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub categories: Option<&'options [&'options str]>,
    pub cursor: Option<&'options str>,
    pub limit: Option<&'options str>,
    pub rule_home: Option<&'options str>,
}

impl crate::Operation for SearchTemplatesWithGet {
    const BASE: crate::Base = crate::Base::Automation;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/v1/template/search";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::SearchTemplatesResponse>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts("categories", options.categories);
        request.query_text("cursor", options.cursor);
        request.query_text("limit", options.limit);
        request.query_text("ruleHome", options.rule_home);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for SearchTemplatesWithGet {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "cursor", items: "/data", next: crate::Next::Link("/links/next") };

    type Item = crate::Object<super::shape::Template>;
}
