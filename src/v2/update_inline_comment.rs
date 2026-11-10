#[derive(Clone, Copy, Debug)]
pub struct UpdateInlineComment<'call> {
    pub body: &'call serde_json::Value,
    pub comment_id: i64,
}

impl crate::Operation for UpdateInlineComment<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/wiki/api/v2/inline-comments/{comment-id}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::UpdateInlineCommentReply>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("comment-id", self.comment_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
