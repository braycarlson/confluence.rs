#[derive(Clone, Copy, Debug)]
pub struct GetTheme<'call> {
    pub theme_key: &'call str,
}

impl crate::Operation for GetTheme<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/settings/theme/{themeKey}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::Theme>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("themeKey", self.theme_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
