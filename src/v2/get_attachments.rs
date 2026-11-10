#[derive(Clone, Copy, Debug)]
pub struct GetAttachments;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub cursor: Option<&'options str>,
    pub filename: Option<&'options str>,
    pub limit: Option<i64>,
    pub media_type: Option<&'options str>,
    pub sort: Option<&'options str>,
    pub status: Option<&'options [&'options str]>,
}

impl crate::Operation for GetAttachments {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/attachments";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::GetAttachmentsReply>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("cursor", options.cursor);
        request.query_text("filename", options.filename);
        request.query_number("limit", options.limit);
        request.query_text("mediaType", options.media_type);
        request.query_text("sort", options.sort);
        request.query_texts("status", options.status);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetAttachments {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "cursor",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::AttachmentBulk>;
}
