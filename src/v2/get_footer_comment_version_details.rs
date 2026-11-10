#[derive(Clone, Copy, Debug)]
pub struct GetFooterCommentVersionDetails {
    pub id: i64,
    pub version_number: i64,
}

impl crate::Operation for GetFooterCommentVersionDetails {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/footer-comments/{id}/versions/{version-number}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::DetailedVersion>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;
        request.path_number("version-number", self.version_number)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
