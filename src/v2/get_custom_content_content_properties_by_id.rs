#[derive(Clone, Copy, Debug)]
pub struct GetCustomContentContentPropertiesByID {
    pub custom_content_id: i64,
    pub property_id: i64,
}

impl crate::Operation for GetCustomContentContentPropertiesByID {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;

    const PATH: &'static str =
        "/wiki/api/v2/custom-content/{custom-content-id}/properties/{property-id}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::ContentProperty>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("custom-content-id", self.custom_content_id)?;
        request.path_number("property-id", self.property_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
