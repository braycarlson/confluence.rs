#[derive(Clone, Copy, Debug)]
pub struct GetViewers<'call> {
    pub content_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub from_date: Option<&'options str>,
}

impl crate::Operation for GetViewers<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/analytics/content/{contentId}/viewers";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::GetViewersReply>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("contentId", self.content_id)?;
        request.query_text("fromDate", options.from_date);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
