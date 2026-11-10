#[derive(Clone, Copy, Debug)]
pub struct GetLookAndFeelSettings;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub space_key: Option<&'options str>,
}

impl crate::Operation for GetLookAndFeelSettings {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/settings/lookandfeel";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::LookAndFeelSettings>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("spaceKey", options.space_key);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
