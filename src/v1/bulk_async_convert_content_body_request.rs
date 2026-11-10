#[derive(Clone, Copy, Debug)]
pub struct BulkAsyncConvertContentBodyRequest<'call> {
    pub body: &'call serde_json::Value,
}

impl crate::Operation for BulkAsyncConvertContentBodyRequest<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/wiki/rest/api/contentbody/convert/async/bulk/tasks";

    type Options<'options> = ();
    type Reply = crate::List<crate::Object<super::shape::AsyncID>>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
