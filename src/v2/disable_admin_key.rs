#[derive(Clone, Copy, Debug)]
pub struct DisableAdminKey;

impl crate::Operation for DisableAdminKey {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/wiki/api/v2/admin-key";

    type Options<'options> = ();
    type Reply = crate::Nothing;

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
