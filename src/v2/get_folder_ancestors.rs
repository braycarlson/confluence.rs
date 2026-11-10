#[derive(Clone, Copy, Debug)]
pub struct GetFolderAncestors {
    pub id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub limit: Option<i64>,
}

impl crate::Operation for GetFolderAncestors {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/folders/{id}/ancestors";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::GetFolderAncestorsReply>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;
        request.query_number("limit", options.limit);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
