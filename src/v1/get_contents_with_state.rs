#[derive(Clone, Copy, Debug)]
pub struct GetContentsWithState<'call> {
    pub space_key: &'call str,
    pub state_id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options [&'options str]>,
    pub limit: Option<i64>,
    pub start: Option<i64>,
}

impl crate::Operation for GetContentsWithState<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/space/{spaceKey}/state/content";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::ContentArray>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts_joined("expand", options.expand)?;
        request.query_number("limit", options.limit);
        request.path_text("spaceKey", self.space_key)?;
        request.query_number("start", options.start);
        request.query_number("state-id", Some(self.state_id));

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetContentsWithState<'_> {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "start",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::Content>;
}
