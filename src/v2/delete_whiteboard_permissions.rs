#[derive(Clone, Copy, Debug)]
pub struct DeleteWhiteboardPermissions {
    pub id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub allow_self_lockout: Option<bool>,
    pub operation: Option<&'options str>,
    pub principal_id: Option<&'options str>,
    pub principal_type: Option<&'options str>,
}

impl crate::Operation for DeleteWhiteboardPermissions {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/wiki/api/v2/whiteboards/{id}/permissions";

    type Options<'options> = Options<'options>;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("allow-self-lockout", options.allow_self_lockout);
        request.path_number("id", self.id)?;
        request.query_text("operation", options.operation);
        request.query_text("principal-id", options.principal_id);
        request.query_text("principal-type", options.principal_type);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
