#[derive(Clone, Copy, Debug)]
pub struct GetAndConvertMacroBodyByMacroID<'call> {
    pub id: &'call str,
    pub macro_id: &'call str,
    pub to_value: &'call str,
    pub version: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub embedded_content_render: Option<&'options str>,
    pub expand: Option<&'options [&'options str]>,
    pub space_key_context: Option<&'options str>,
}

impl crate::Operation for GetAndConvertMacroBodyByMacroID<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;

    const PATH: &'static str =
        "/wiki/rest/api/content/{id}/history/{version}/macro/id/{macroId}/convert/{to}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::ContentBody>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("embeddedContentRender", options.embedded_content_render);
        request.query_texts_joined("expand", options.expand)?;
        request.path_text("id", self.id)?;
        request.path_text("macroId", self.macro_id)?;
        request.query_text("spaceKeyContext", options.space_key_context);
        request.path_text("to", self.to_value)?;
        request.path_number("version", self.version)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
