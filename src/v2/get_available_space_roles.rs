#[derive(Clone, Copy, Debug)]
pub struct GetAvailableSpaceRoles;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub cursor: Option<&'options str>,
    pub limit: Option<i64>,
    pub principal_id: Option<&'options str>,
    pub principal_type: Option<&'options str>,
    pub role_type: Option<&'options str>,
    pub space_id: Option<&'options str>,
}

impl crate::Operation for GetAvailableSpaceRoles {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/space-roles";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::GetAvailableSpaceRolesReply>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("cursor", options.cursor);
        request.query_number("limit", options.limit);
        request.query_text("principal-id", options.principal_id);
        request.query_text("principal-type", options.principal_type);
        request.query_text("role-type", options.role_type);
        request.query_text("space-id", options.space_id);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetAvailableSpaceRoles {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "cursor",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::SpaceRole>;
}
