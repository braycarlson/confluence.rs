#[derive(Clone, Copy, Debug)]
pub struct GetLabelsForSpace<'call> {
    pub space_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub limit: Option<i64>,
    pub prefix: Option<&'options str>,
    pub start: Option<i64>,
}

impl crate::Operation for GetLabelsForSpace<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/space/{spaceKey}/label";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::LabelArray>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_number("limit", options.limit);
        request.query_text("prefix", options.prefix);
        request.path_text("spaceKey", self.space_key)?;
        request.query_number("start", options.start);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetLabelsForSpace<'_> {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "start",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::Label>;
}
