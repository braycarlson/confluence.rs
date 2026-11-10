#[derive(Clone, Copy, Debug)]
pub struct CreateDatabase<'call> {
    pub body: &'call serde_json::Value,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub private: Option<bool>,
}

impl crate::Operation for CreateDatabase<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/wiki/api/v2/databases";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::CreateDatabaseReply>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("private", options.private);
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
