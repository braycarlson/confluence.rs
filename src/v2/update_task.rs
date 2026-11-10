#[derive(Clone, Copy, Debug)]
pub struct UpdateTask<'call> {
    pub body: &'call serde_json::Value,
    pub id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub body_format: Option<&'options str>,
}

impl crate::Operation for UpdateTask<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/wiki/api/v2/tasks/{id}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::Task>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("body-format", options.body_format);
        request.path_number("id", self.id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
