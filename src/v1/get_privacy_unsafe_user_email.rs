#[derive(Clone, Copy, Debug)]
pub struct GetPrivacyUnsafeUserEmail<'call> {
    pub account_id: &'call str,
}

impl crate::Operation for GetPrivacyUnsafeUserEmail<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/user/email";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::AccountIDEmailRecord>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("accountId", Some(self.account_id));

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
