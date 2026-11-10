#[derive(Clone, Copy, Debug)]
pub struct AsyncConvertContentBodyRequest<'call> {
    pub body: &'call serde_json::Value,
    pub to_value: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub allow_cache: Option<bool>,
    pub content_id_context: Option<&'options str>,
    pub embedded_content_render: Option<&'options str>,
    pub expand: Option<&'options [&'options str]>,
    pub space_key_context: Option<&'options str>,
}

impl crate::Operation for AsyncConvertContentBodyRequest<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/wiki/rest/api/contentbody/convert/async/{to}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::AsyncID>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("allowCache", options.allow_cache);
        request.query_text("contentIdContext", options.content_id_context);
        request.query_text("embeddedContentRender", options.embedded_content_render);
        request.query_texts_joined("expand", options.expand)?;
        request.query_text("spaceKeyContext", options.space_key_context);
        request.path_text("to", self.to_value)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
