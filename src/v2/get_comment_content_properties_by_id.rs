#[derive(Clone, Copy, Debug)]
pub struct GetCommentContentPropertiesByID {
    pub comment_id: i64,
    pub property_id: i64,
}

impl crate::Operation for GetCommentContentPropertiesByID {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/comments/{comment-id}/properties/{property-id}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::ContentProperty>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("comment-id", self.comment_id)?;
        request.path_number("property-id", self.property_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
