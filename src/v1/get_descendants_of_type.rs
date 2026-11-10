#[derive(Clone, Copy, Debug)]
pub struct GetDescendantsOfType<'call> {
    pub id: &'call str,
    pub kind: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub depth: Option<&'options str>,
    pub expand: Option<&'options [&'options str]>,
    pub limit: Option<i64>,
    pub start: Option<i64>,
}

impl crate::Operation for GetDescendantsOfType<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/content/{id}/descendant/{type}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::ContentArray>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("depth", options.depth);
        request.query_texts_joined("expand", options.expand)?;
        request.path_text("id", self.id)?;
        request.path_text("type", self.kind)?;
        request.query_number("limit", options.limit);
        request.query_number("start", options.start);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetDescendantsOfType<'_> {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "start",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::Content>;
}
