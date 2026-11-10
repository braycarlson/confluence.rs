#[derive(Clone, Copy, Debug)]
pub struct SearchContentByCQL<'call> {
    pub cql: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub cqlcontext: Option<&'options str>,
    pub cursor: Option<&'options str>,
    pub expand: Option<&'options [&'options str]>,
    pub limit: Option<i64>,
}

impl crate::Operation for SearchContentByCQL<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/content/search";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::ContentArray>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("cql", Some(self.cql));
        request.query_text("cqlcontext", options.cqlcontext);
        request.query_text("cursor", options.cursor);
        request.query_texts_joined("expand", options.expand)?;
        request.query_number("limit", options.limit);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for SearchContentByCQL<'_> {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "cursor",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::Content>;
}
