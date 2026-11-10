#[derive(Clone, Copy, Debug)]
pub struct ResetSpaceTheme<'call> {
    pub space_key: &'call str,
}

impl crate::Operation for ResetSpaceTheme<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/wiki/rest/api/space/{spaceKey}/theme";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("spaceKey", self.space_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
