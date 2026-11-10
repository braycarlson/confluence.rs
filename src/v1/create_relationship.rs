#[derive(Clone, Copy, Debug)]
pub struct CreateRelationship<'call> {
    pub relation_name: &'call str,
    pub source_key: &'call str,
    pub source_type: &'call str,
    pub target_key: &'call str,
    pub target_type: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub source_status: Option<&'options str>,
    pub source_version: Option<i64>,
    pub target_status: Option<&'options str>,
    pub target_version: Option<i64>,
}

impl crate::Operation for CreateRelationship<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;

    const PATH: &'static str = concat!(
        "/wiki/rest/api/relation/{relationName}/from/{sourceType}/{sourceKey}/to/{targetType}/",
        "{targetKey}",
    );

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::Relation>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("relationName", self.relation_name)?;
        request.path_text("sourceKey", self.source_key)?;
        request.query_text("sourceStatus", options.source_status);
        request.path_text("sourceType", self.source_type)?;
        request.query_number("sourceVersion", options.source_version);
        request.path_text("targetKey", self.target_key)?;
        request.query_text("targetStatus", options.target_status);
        request.path_text("targetType", self.target_type)?;
        request.query_number("targetVersion", options.target_version);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
