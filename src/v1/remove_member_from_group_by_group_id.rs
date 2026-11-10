#[derive(Clone, Copy, Debug)]
pub struct RemoveMemberFromGroupByGroupID<'call> {
    pub account_id: &'call str,
    pub group_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub key: Option<&'options str>,
    pub username: Option<&'options str>,
}

impl crate::Operation for RemoveMemberFromGroupByGroupID<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/wiki/rest/api/group/userByGroupId";

    type Options<'options> = Options<'options>;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("accountId", Some(self.account_id));
        request.query_text("groupId", Some(self.group_id));
        request.query_text("key", options.key);
        request.query_text("username", options.username);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
