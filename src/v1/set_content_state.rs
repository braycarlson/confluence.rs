#[derive(Clone, Copy, Debug)]
pub struct SetContentState<'call> {
    pub body: &'call serde_json::Value,
    pub id: &'call str,
    pub status: &'call str,
}

impl crate::Operation for SetContentState<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/wiki/rest/api/content/{id}/state";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::ContentStateResponse>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;
        request.query_text("status", Some(self.status));
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
