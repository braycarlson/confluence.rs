#[derive(Clone, Copy, Debug)]
pub struct AsyncConvertContentBodyResponse<'call> {
    pub id: &'call str,
}

impl crate::Operation for AsyncConvertContentBodyResponse<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/contentbody/convert/async/{id}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::AsyncContentBody>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
