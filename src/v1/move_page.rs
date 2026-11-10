#[derive(Clone, Copy, Debug)]
pub struct MovePage<'call> {
    pub page_id: &'call str,
    pub position: &'call str,
    pub target_id: &'call str,
}

impl crate::Operation for MovePage<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/wiki/rest/api/content/{pageId}/move/{position}/{targetId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::MovePageReply>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("pageId", self.page_id)?;
        request.path_text("position", self.position)?;
        request.path_text("targetId", self.target_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
