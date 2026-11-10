#[derive(Clone, Copy, Debug)]
pub struct CreateRuleFromTemplate;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub body: Option<&'options serde_json::Value>,
}

impl crate::Operation for CreateRuleFromTemplate {
    const BASE: crate::Base = crate::Base::Automation;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/v1/template/create";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::CreateRuleFromTemplateResponse>;

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
