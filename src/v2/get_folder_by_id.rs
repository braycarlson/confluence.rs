#[derive(Clone, Copy, Debug)]
pub struct GetFolderByID {
    pub id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub include_collaborators: Option<bool>,
    pub include_direct_children: Option<bool>,
    pub include_operations: Option<bool>,
    pub include_properties: Option<bool>,
}

impl crate::Operation for GetFolderByID {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/folders/{id}";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::GetFolderByIDReply>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;
        request.query_boolean("include-collaborators", options.include_collaborators);
        request.query_boolean("include-direct-children", options.include_direct_children);
        request.query_boolean("include-operations", options.include_operations);
        request.query_boolean("include-properties", options.include_properties);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
