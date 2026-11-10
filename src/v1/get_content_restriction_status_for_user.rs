#[derive(Clone, Copy, Debug)]
pub struct GetContentRestrictionStatusForUser<'call> {
    pub id: &'call str,
    pub operation_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub account_id: Option<&'options str>,
    pub key: Option<&'options str>,
    pub username: Option<&'options str>,
}

impl crate::Operation for GetContentRestrictionStatusForUser<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;

    const PATH: &'static str =
        "/wiki/rest/api/content/{id}/restriction/byOperation/{operationKey}/user";

    type Options<'options> = Options<'options>;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("accountId", options.account_id);
        request.path_text("id", self.id)?;
        request.query_text("key", options.key);
        request.path_text("operationKey", self.operation_key)?;
        request.query_text("username", options.username);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
