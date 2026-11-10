#[derive(Clone, Copy, Debug)]
pub struct SearchManualRulesWithPost;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub body: Option<&'options serde_json::Value>,
}

impl crate::Operation for SearchManualRulesWithPost {
    const BASE: crate::Base = crate::Base::Automation;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/v1/rule/manual/search";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::SearchManualRuleResponse>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        if let Some(body) = options.body {
            request.body_json(body)?;
        }

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
