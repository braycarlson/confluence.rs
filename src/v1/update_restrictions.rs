#[derive(Clone, Copy, Debug)]
pub struct UpdateRestrictions<'call> {
    pub body: &'call serde_json::Value,
    pub id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options [&'options str]>,
}

impl crate::Operation for UpdateRestrictions<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/wiki/rest/api/content/{id}/restriction";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::ContentRestrictionArray>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts_joined("expand", options.expand)?;
        request.path_text("id", self.id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
