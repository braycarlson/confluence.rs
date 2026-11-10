#[derive(Clone, Copy, Debug)]
pub struct ListSpacePermissionCombinations;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub cursor: Option<&'options str>,
    pub limit: Option<i64>,
}

impl crate::Operation for ListSpacePermissionCombinations {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/space-permissions/transition/combinations";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::ListSpacePermissionCombinationsResponse>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("cursor", options.cursor);
        request.query_number("limit", options.limit);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for ListSpacePermissionCombinations {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "cursor", items: "/results", next: crate::Next::Token("/cursor") };

    type Item = crate::Object<super::shape::BulkTransitionCombinationEntry>;
}
