#[derive(Clone, Copy, Debug)]
pub struct DeleteLabelFromSpace<'call> {
    pub name: &'call str,
    pub space_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub prefix: Option<&'options str>,
}

impl crate::Operation for DeleteLabelFromSpace<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/wiki/rest/api/space/{spaceKey}/label";

    type Options<'options> = Options<'options>;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("name", Some(self.name));
        request.query_text("prefix", options.prefix);
        request.path_text("spaceKey", self.space_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
