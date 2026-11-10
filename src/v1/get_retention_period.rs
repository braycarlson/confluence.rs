#[derive(Clone, Copy, Debug)]
pub struct GetRetentionPeriod;

impl crate::Operation for GetRetentionPeriod {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/audit/retention";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::RetentionPeriod>;

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
