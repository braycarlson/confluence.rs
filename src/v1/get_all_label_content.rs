#[derive(Clone, Copy, Debug)]
pub struct GetAllLabelContent<'call> {
    pub name: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub kind: Option<&'options str>,
    pub limit: Option<i64>,
    pub start: Option<i64>,
}

impl crate::Operation for GetAllLabelContent<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/label";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::LabelDetails>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("type", options.kind);
        request.query_number("limit", options.limit);
        request.query_text("name", Some(self.name));
        request.query_number("start", options.start);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
