#[derive(Clone, Copy, Debug)]
pub struct PublishLegacyDraft<'call> {
    pub body: &'call serde_json::Value,
    pub draft_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options [&'options str]>,
    pub status: Option<&'options str>,
}

impl crate::Operation for PublishLegacyDraft<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/wiki/rest/api/content/blueprint/instance/{draftId}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::Content>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("draftId", self.draft_id)?;
        request.query_texts_joined("expand", options.expand)?;
        request.query_text("status", options.status);
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
