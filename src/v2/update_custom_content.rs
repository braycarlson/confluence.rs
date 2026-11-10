#[derive(Clone, Copy, Debug)]
pub struct UpdateCustomContent<'call> {
    pub body: &'call serde_json::Value,
    pub id: i64,
}

impl crate::Operation for UpdateCustomContent<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/wiki/api/v2/custom-content/{id}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::UpdateCustomContentReply>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
