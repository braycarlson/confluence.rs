#[derive(Clone, Copy, Debug)]
pub struct GetAttachmentThumbnailByID<'call> {
    pub id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub height: Option<i64>,
    pub version: Option<i64>,
    pub width: Option<i64>,
}

impl crate::Operation for GetAttachmentThumbnailByID<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/attachments/{id}/thumbnail/download";

    type Options<'options> = Options;
    type Reply = crate::Binary;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_number("height", options.height);
        request.path_text("id", self.id)?;
        request.query_number("version", options.version);
        request.query_number("width", options.width);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
