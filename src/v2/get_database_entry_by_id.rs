#[derive(Clone, Copy, Debug)]
pub struct GetDatabaseEntryByID<'call> {
    pub entry_id: &'call str,
    pub id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub if_none_match: Option<&'options str>,
}

impl crate::Operation for GetDatabaseEntryByID<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/databases/{id}/entries/{entryId}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::DatabaseEntrySingleResponse>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("entryId", self.entry_id)?;
        request.path_text("id", self.id)?;
        request.header_text("If-None-Match", options.if_none_match)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
