#[derive(Clone, Copy, Debug)]
pub struct CreateCustomContentProperty<'call> {
    pub body: &'call serde_json::Value,
    pub custom_content_id: i64,
}

impl crate::Operation for CreateCustomContentProperty<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/wiki/api/v2/custom-content/{custom-content-id}/properties";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::ContentProperty>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("custom-content-id", self.custom_content_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
