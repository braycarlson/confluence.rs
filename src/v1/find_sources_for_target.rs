#[derive(Clone, Copy, Debug)]
pub struct FindSourcesForTarget<'call> {
    pub relation_name: &'call str,
    pub source_type: &'call str,
    pub target_key: &'call str,
    pub target_type: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options [&'options str]>,
    pub limit: Option<i64>,
    pub source_status: Option<&'options str>,
    pub source_version: Option<i64>,
    pub start: Option<i64>,
    pub target_status: Option<&'options str>,
    pub target_version: Option<i64>,
}

impl crate::Operation for FindSourcesForTarget<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;

    const PATH: &'static str =
        "/wiki/rest/api/relation/{relationName}/to/{targetType}/{targetKey}/from/{sourceType}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::RelationArray>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts_joined("expand", options.expand)?;
        request.query_number("limit", options.limit);
        request.path_text("relationName", self.relation_name)?;
        request.query_text("sourceStatus", options.source_status);
        request.path_text("sourceType", self.source_type)?;
        request.query_number("sourceVersion", options.source_version);
        request.query_number("start", options.start);
        request.path_text("targetKey", self.target_key)?;
        request.query_text("targetStatus", options.target_status);
        request.path_text("targetType", self.target_type)?;
        request.query_number("targetVersion", options.target_version);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for FindSourcesForTarget<'_> {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "start",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::Relation>;
}
