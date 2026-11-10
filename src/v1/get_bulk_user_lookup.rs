#[derive(Clone, Copy, Debug)]
pub struct GetBulkUserLookup<'call> {
    pub account_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options [&'options str]>,
}

impl crate::Operation for GetBulkUserLookup<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/user/bulk";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::BulkUserLookupArray>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("accountId", Some(self.account_id));
        request.query_texts_joined("expand", options.expand)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
