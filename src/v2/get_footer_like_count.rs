#[derive(Clone, Copy, Debug)]
pub struct GetFooterLikeCount {
    pub id: i64,
}

impl crate::Operation for GetFooterLikeCount {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/footer-comments/{id}/likes/count";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::GetFooterLikeCountReply>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
