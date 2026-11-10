#[derive(Clone, Copy, Debug)]
pub struct GetBlogPostClassificationLevel {
    pub id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub status: Option<&'options str>,
}

impl crate::Operation for GetBlogPostClassificationLevel {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/blogposts/{id}/classification-level";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::ClassificationLevel>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;
        request.query_text("status", options.status);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
