#[derive(Clone, Copy, Debug)]
pub struct GetFooterCommentByID {
    pub comment_id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub body_format: Option<&'options str>,
    pub include_likes: Option<bool>,
    pub include_operations: Option<bool>,
    pub include_properties: Option<bool>,
    pub include_version: Option<bool>,
    pub include_versions: Option<bool>,
    pub version: Option<i64>,
}

impl crate::Operation for GetFooterCommentByID {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/footer-comments/{comment-id}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::GetFooterCommentByIDReply>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("body-format", options.body_format);
        request.path_number("comment-id", self.comment_id)?;
        request.query_boolean("include-likes", options.include_likes);
        request.query_boolean("include-operations", options.include_operations);
        request.query_boolean("include-properties", options.include_properties);
        request.query_boolean("include-version", options.include_version);
        request.query_boolean("include-versions", options.include_versions);
        request.query_number("version", options.version);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
