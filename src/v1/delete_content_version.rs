#[derive(Clone, Copy, Debug)]
pub struct DeleteContentVersion<'call> {
    pub id: &'call str,
    pub version_number: i64,
}

impl crate::Operation for DeleteContentVersion<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/wiki/rest/api/content/{id}/version/{versionNumber}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;
        request.path_number("versionNumber", self.version_number)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
