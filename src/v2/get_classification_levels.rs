#[derive(Clone, Copy, Debug)]
pub struct GetClassificationLevels;

impl crate::Operation for GetClassificationLevels {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/classification-levels";

    type Options<'options> = ();
    type Reply = crate::List<crate::Object<super::shape::ClassificationLevel>>;

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
