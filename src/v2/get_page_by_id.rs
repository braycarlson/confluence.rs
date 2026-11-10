#[derive(Clone, Copy, Debug)]
pub struct GetPageByID {
    pub id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub body_format: Option<&'options str>,
    pub get_draft: Option<bool>,
    pub include_collaborators: Option<bool>,
    pub include_direct_children: Option<bool>,
    pub include_favorited_by_user_status_current: Option<bool>,
    pub include_labels: Option<bool>,
    pub include_likes: Option<bool>,
    pub include_operations: Option<bool>,
    pub include_properties: Option<bool>,
    pub include_version: Option<bool>,
    pub include_versions: Option<bool>,
    pub include_webresources: Option<bool>,
    pub status: Option<&'options [&'options str]>,
    pub version: Option<i64>,
}

impl crate::Operation for GetPageByID {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/pages/{id}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::GetPageByIDReply>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("body-format", options.body_format);
        request.query_boolean("get-draft", options.get_draft);
        request.path_number("id", self.id)?;
        request.query_boolean("include-collaborators", options.include_collaborators);
        request.query_boolean("include-direct-children", options.include_direct_children);

        request.query_boolean(
            "include-favorited-by-current-user-status",
            options.include_favorited_by_user_status_current,
        );

        request.query_boolean("include-labels", options.include_labels);
        request.query_boolean("include-likes", options.include_likes);
        request.query_boolean("include-operations", options.include_operations);
        request.query_boolean("include-properties", options.include_properties);
        request.query_boolean("include-version", options.include_version);
        request.query_boolean("include-versions", options.include_versions);
        request.query_boolean("include-webresources", options.include_webresources);
        request.query_texts("status", options.status);
        request.query_number("version", options.version);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
