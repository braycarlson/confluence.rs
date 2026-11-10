#[derive(Clone, Copy, Debug)]
pub struct GetFolderDescendants {
    pub id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub cursor: Option<&'options str>,
    pub depth: Option<i64>,
    pub limit: Option<i64>,
}

impl crate::Operation for GetFolderDescendants {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/folders/{id}/descendants";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::GetFolderDescendantsReply>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("cursor", options.cursor);
        request.query_number("depth", options.depth);
        request.path_number("id", self.id)?;
        request.query_number("limit", options.limit);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetFolderDescendants {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "cursor",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::DescendantsResponse>;
}
