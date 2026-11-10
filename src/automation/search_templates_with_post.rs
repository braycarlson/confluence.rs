#[derive(Clone, Copy, Debug)]
pub struct SearchTemplatesWithPost;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub body: Option<&'options serde_json::Value>,
}

impl crate::Operation for SearchTemplatesWithPost {
    const BASE: crate::Base = crate::Base::Automation;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/v1/template/search";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::SearchTemplatesResponse>;

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
