#[derive(Clone, Copy, Debug)]
pub struct GetSpaceByID {
    pub id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub description_format: Option<&'options str>,
    pub include_icon: Option<bool>,
    pub include_labels: Option<bool>,
    pub include_operations: Option<bool>,
    pub include_permissions: Option<bool>,
    pub include_properties: Option<bool>,
    pub include_role_assignments: Option<bool>,
}

impl crate::Operation for GetSpaceByID {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/spaces/{id}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::GetSpaceByIDReply>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("description-format", options.description_format);
        request.path_number("id", self.id)?;
        request.query_boolean("include-icon", options.include_icon);
        request.query_boolean("include-labels", options.include_labels);
        request.query_boolean("include-operations", options.include_operations);
        request.query_boolean("include-permissions", options.include_permissions);
        request.query_boolean("include-properties", options.include_properties);
        request.query_boolean("include-role-assignments", options.include_role_assignments);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
