#[derive(Clone, Copy, Debug)]
pub struct SearchManualRulesWithGet<'call> {
    pub cursor: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub limit: Option<&'options str>,
}

impl crate::Operation for SearchManualRulesWithGet<'_> {
    const BASE: crate::Base = crate::Base::Automation;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/v1/rule/manual/search";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::SearchManualRuleResponse>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("cursor", Some(self.cursor));
        request.query_text("limit", options.limit);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for SearchManualRulesWithGet<'_> {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "cursor", items: "/data", next: crate::Next::Link("/links/next") };

    type Item = crate::Object<super::shape::ManualTriggerRule>;
}
