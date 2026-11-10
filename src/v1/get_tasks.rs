#[derive(Clone, Copy, Debug)]
pub struct GetTasks;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub key: Option<&'options str>,
    pub limit: Option<i64>,
    pub start: Option<i64>,
}

impl crate::Operation for GetTasks {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/longtask";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::LongTaskStatusArray>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("key", options.key);
        request.query_number("limit", options.limit);
        request.query_number("start", options.start);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetTasks {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "start",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::LongTaskStatus>;
}
