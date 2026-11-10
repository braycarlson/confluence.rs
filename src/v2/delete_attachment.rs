#[derive(Clone, Copy, Debug)]
pub struct DeleteAttachment {
    pub id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub purge: Option<bool>,
}

impl crate::Operation for DeleteAttachment {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/wiki/api/v2/attachments/{id}";

    type Options<'options> = Options;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;
        request.query_boolean("purge", options.purge);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
