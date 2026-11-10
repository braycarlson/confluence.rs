#[derive(Clone, Copy, Debug)]
pub struct GetAttachmentOperations<'call> {
    pub id: &'call str,
}

impl crate::Operation for GetAttachmentOperations<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/attachments/{id}/operations";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::PermittedOperationsResponse>;

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
