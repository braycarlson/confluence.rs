#[derive(Clone, Copy, Debug)]
pub struct GetPagesInSpace {
    pub id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub body_format: Option<&'options str>,
    pub cursor: Option<&'options str>,
    pub depth: Option<&'options str>,
    pub limit: Option<i64>,
    pub sort: Option<&'options str>,
    pub status: Option<&'options [&'options str]>,
    pub title: Option<&'options str>,
}

impl crate::Operation for GetPagesInSpace {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/spaces/{id}/pages";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::GetPagesInSpaceReply>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("body-format", options.body_format);
        request.query_text("cursor", options.cursor);
        request.query_text("depth", options.depth);
        request.path_number("id", self.id)?;
        request.query_number("limit", options.limit);
        request.query_text("sort", options.sort);
        request.query_texts("status", options.status);
        request.query_text("title", options.title);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetPagesInSpace {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "cursor",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::PageBulk>;
}
