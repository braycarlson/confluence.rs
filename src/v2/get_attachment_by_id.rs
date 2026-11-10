#[derive(Clone, Copy, Debug)]
pub struct GetAttachmentByID<'call> {
    pub id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub include_collaborators: Option<bool>,
    pub include_labels: Option<bool>,
    pub include_operations: Option<bool>,
    pub include_properties: Option<bool>,
    pub include_version: Option<bool>,
    pub include_versions: Option<bool>,
    pub version: Option<i64>,
}

impl crate::Operation for GetAttachmentByID<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/attachments/{id}";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::GetAttachmentByIDReply>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;
        request.query_boolean("include-collaborators", options.include_collaborators);
        request.query_boolean("include-labels", options.include_labels);
        request.query_boolean("include-operations", options.include_operations);
        request.query_boolean("include-properties", options.include_properties);
        request.query_boolean("include-version", options.include_version);
        request.query_boolean("include-versions", options.include_versions);
        request.query_number("version", options.version);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
