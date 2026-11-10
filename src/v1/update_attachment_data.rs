#[derive(Clone, Copy, Debug)]
pub struct UpdateAttachmentData<'call> {
    pub attachment_id: &'call str,
    pub body: &'call [crate::Part<'call>],
    pub id: &'call str,
}

impl crate::Operation for UpdateAttachmentData<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/wiki/rest/api/content/{id}/child/attachment/{attachmentId}/data";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::Content>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("attachmentId", self.attachment_id)?;
        request.path_text("id", self.id)?;
        request.body_multipart(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
