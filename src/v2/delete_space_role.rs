#[derive(Clone, Copy, Debug)]
pub struct DeleteSpaceRole<'call> {
    pub id: &'call str,
}

impl crate::Operation for DeleteSpaceRole<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/wiki/api/v2/space-roles/{id}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::DeleteSpaceRoleResponse>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
