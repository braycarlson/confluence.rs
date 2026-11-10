#[derive(Clone, Copy, Debug)]
pub struct DeleteUserProperty<'call> {
    pub key: &'call str,
    pub user_id: &'call str,
}

impl crate::Operation for DeleteUserProperty<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/wiki/rest/api/user/{userId}/property/{key}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("key", self.key)?;
        request.path_text("userId", self.user_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
