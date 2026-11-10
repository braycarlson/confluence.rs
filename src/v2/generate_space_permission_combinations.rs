#[derive(Clone, Copy, Debug)]
pub struct GenerateSpacePermissionCombinations;

impl crate::Operation for GenerateSpacePermissionCombinations {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/wiki/api/v2/space-permissions/transition/combinations";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::BulkTransitionTaskResponse>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
