#[derive(Clone, Copy, Debug)]
pub struct CreateAttachment<'call> {
    pub body: &'call [crate::Part<'call>],
    pub id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub status: Option<&'options str>,
}

impl crate::Operation for CreateAttachment<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/wiki/rest/api/content/{id}/child/attachment";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::ContentArray>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;
        request.query_text("status", options.status);
        request.body_multipart(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
