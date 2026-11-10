#[derive(Clone, Copy, Debug)]
pub struct AddPermissionToSpace<'call> {
    pub body: &'call serde_json::Value,
    pub space_key: &'call str,
}

impl crate::Operation for AddPermissionToSpace<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/wiki/rest/api/space/{spaceKey}/permission";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::SpacePermissionV2>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("spaceKey", self.space_key)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
