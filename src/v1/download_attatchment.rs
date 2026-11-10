#[derive(Clone, Copy, Debug)]
pub struct DownloadAttatchment<'call> {
    pub attachment_id: &'call str,
    pub id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub status: Option<&'options [&'options str]>,
    pub version: Option<i64>,
}

impl crate::Operation for DownloadAttatchment<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;

    const PATH: &'static str =
        "/wiki/rest/api/content/{id}/child/attachment/{attachmentId}/download";

    type Options<'options> = Options<'options>;
    type Reply = crate::Binary;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("attachmentId", self.attachment_id)?;
        request.path_text("id", self.id)?;
        request.query_texts("status", options.status);
        request.query_number("version", options.version);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
