#[derive(Clone, Copy, Debug)]
pub struct CheckContentPermission<'call> {
    pub body: &'call serde_json::Value,
    pub id: &'call str,
}

impl crate::Operation for CheckContentPermission<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/wiki/rest/api/content/{id}/permission/check";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::PermissionCheckResponse>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
