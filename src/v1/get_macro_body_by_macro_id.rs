#[derive(Clone, Copy, Debug)]
pub struct GetMacroBodyByMacroID<'call> {
    pub id: &'call str,
    pub macro_id: &'call str,
    pub version: i64,
}

impl crate::Operation for GetMacroBodyByMacroID<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/content/{id}/history/{version}/macro/id/{macroId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::MacroInstance>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;
        request.path_text("macroId", self.macro_id)?;
        request.path_number("version", self.version)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
