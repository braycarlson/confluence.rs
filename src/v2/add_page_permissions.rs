#[derive(Clone, Copy, Debug)]
pub struct AddPagePermissions<'call> {
    pub body: &'call serde_json::Value,
    pub id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub allow_self_lockout: Option<bool>,
}

impl crate::Operation for AddPagePermissions<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/wiki/api/v2/pages/{id}/permissions";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::ContentPermissionsResponse>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("allow-self-lockout", options.allow_self_lockout);
        request.path_number("id", self.id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
