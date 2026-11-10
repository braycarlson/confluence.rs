#[derive(Clone, Copy, Debug)]
pub struct PutForgeAppProperty<'call> {
    pub body: &'call serde_json::Value,
    pub property_key: &'call str,
}

impl crate::Operation for PutForgeAppProperty<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/wiki/api/v2/app/properties/{propertyKey}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("propertyKey", self.property_key)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
