#[derive(Clone, Copy, Debug)]
pub struct UpdateDatabasePropertyByID<'call> {
    pub body: &'call serde_json::Value,
    pub database_id: i64,
    pub property_id: i64,
}

impl crate::Operation for UpdateDatabasePropertyByID<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/wiki/api/v2/databases/{database-id}/properties/{property-id}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::ContentProperty>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("database-id", self.database_id)?;
        request.path_number("property-id", self.property_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
