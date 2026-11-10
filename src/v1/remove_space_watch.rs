#[derive(Clone, Copy, Debug)]
pub struct RemoveSpaceWatch<'call> {
    pub space_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub account_id: Option<&'options str>,
    pub key: Option<&'options str>,
    pub username: Option<&'options str>,
}

impl crate::Operation for RemoveSpaceWatch<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/wiki/rest/api/user/watch/space/{spaceKey}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("accountId", options.account_id);
        request.query_text("key", options.key);
        request.path_text("spaceKey", self.space_key)?;
        request.query_text("username", options.username);
        request.header("X-Atlassian-Token", "no-check");

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
