#[derive(Clone, Copy, Debug)]
pub struct GetRestrictionsForOperation<'call> {
    pub id: &'call str,
    pub operation_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options [&'options str]>,
    pub limit: Option<i64>,
    pub start: Option<i64>,
}

impl crate::Operation for GetRestrictionsForOperation<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/content/{id}/restriction/byOperation/{operationKey}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::ContentRestriction>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts_joined("expand", options.expand)?;
        request.path_text("id", self.id)?;
        request.query_number("limit", options.limit);
        request.path_text("operationKey", self.operation_key)?;
        request.query_number("start", options.start);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
