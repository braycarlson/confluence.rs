#[derive(Clone, Copy, Debug)]
pub struct GetContentState<'call> {
    pub id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub status: Option<&'options str>,
}

impl crate::Operation for GetContentState<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/content/{id}/state";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::ContentStateResponse>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;
        request.query_text("status", options.status);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
