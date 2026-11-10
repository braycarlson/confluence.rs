#[derive(Clone, Copy, Debug)]
pub struct GetContentTemplate<'call> {
    pub content_template_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options [&'options str]>,
}

impl crate::Operation for GetContentTemplate<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/template/{contentTemplateId}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::ContentTemplate>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("contentTemplateId", self.content_template_id)?;
        request.query_texts_joined("expand", options.expand)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
