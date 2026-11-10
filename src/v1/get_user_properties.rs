#[derive(Clone, Copy, Debug)]
pub struct GetUserProperties<'call> {
    pub user_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub limit: Option<i64>,
    pub start: Option<i64>,
}

impl crate::Operation for GetUserProperties<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/user/{userId}/property";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::UserPropertyKeyArray>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_number("limit", options.limit);
        request.query_number("start", options.start);
        request.path_text("userId", self.user_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetUserProperties<'_> {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "start",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::GetUserPropertiesItem>;
}
