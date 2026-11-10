#[derive(Clone, Copy, Debug)]
pub struct GetWatchersForSpace<'call> {
    pub space_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub limit: Option<&'options str>,
    pub start: Option<&'options str>,
}

impl crate::Operation for GetWatchersForSpace<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/space/{spaceKey}/watch";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::SpaceWatchArray>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("limit", options.limit);
        request.path_text("spaceKey", self.space_key)?;
        request.query_text("start", options.start);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetWatchersForSpace<'_> {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "start",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::SpaceWatch>;
}
