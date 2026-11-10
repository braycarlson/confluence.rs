#[derive(Clone, Copy, Debug)]
pub struct BulkAsyncConvertContentBodyResponse<'call> {
    pub ids: &'call [&'call str],
}

impl crate::Operation for BulkAsyncConvertContentBodyResponse<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/contentbody/convert/async/bulk/tasks";

    type Options<'options> = ();
    type Reply = crate::List<crate::Object<super::shape::AsyncContentBody>>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts("ids", Some(self.ids));

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
