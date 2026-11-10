#[derive(Clone, Copy, Debug)]
pub struct GetForgeAppProperty<'call> {
    pub property_key: &'call str,
}

impl crate::Operation for GetForgeAppProperty<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/app/properties/{propertyKey}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::GetForgeAppPropertyReply>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("propertyKey", self.property_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
