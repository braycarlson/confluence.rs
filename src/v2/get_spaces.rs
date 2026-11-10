#[derive(Clone, Copy, Debug)]
pub struct GetSpaces;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub cursor: Option<&'options str>,
    pub description_format: Option<&'options str>,
    pub favorited_by: Option<&'options str>,
    pub ids: Option<&'options [i64]>,
    pub include_icon: Option<bool>,
    pub keys: Option<&'options [&'options str]>,
    pub kind: Option<&'options str>,
    pub labels: Option<&'options [&'options str]>,
    pub limit: Option<i64>,
    pub not_favorited_by: Option<&'options str>,
    pub sort: Option<&'options str>,
    pub status: Option<&'options str>,
}

impl crate::Operation for GetSpaces {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/spaces";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::GetSpacesReply>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("cursor", options.cursor);
        request.query_text("description-format", options.description_format);
        request.query_text("favorited-by", options.favorited_by);
        request.query_numbers("ids", options.ids);
        request.query_boolean("include-icon", options.include_icon);
        request.query_texts("keys", options.keys);
        request.query_text("type", options.kind);
        request.query_texts("labels", options.labels);
        request.query_number("limit", options.limit);
        request.query_text("not-favorited-by", options.not_favorited_by);
        request.query_text("sort", options.sort);
        request.query_text("status", options.status);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetSpaces {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "cursor",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::SpaceBulk>;
}
