#[derive(Clone, Copy, Debug)]
pub struct DeletePagePropertyByID {
    pub page_id: i64,
    pub property_id: i64,
}

impl crate::Operation for DeletePagePropertyByID {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/wiki/api/v2/pages/{page-id}/properties/{property-id}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("page-id", self.page_id)?;
        request.path_number("property-id", self.property_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
