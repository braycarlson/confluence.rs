#[derive(Clone, Copy, Debug)]
pub struct IsWatchingLabel<'call> {
    pub label_name: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub account_id: Option<&'options str>,
    pub key: Option<&'options str>,
    pub username: Option<&'options str>,
}

impl crate::Operation for IsWatchingLabel<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/user/watch/label/{labelName}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::UserWatch>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("accountId", options.account_id);
        request.query_text("key", options.key);
        request.path_text("labelName", self.label_name)?;
        request.query_text("username", options.username);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
