#[derive(Clone, Copy, Debug)]
pub struct GetModules;

impl crate::Operation for GetModules {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/atlassian-connect/1/app/module/dynamic";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::ConnectModules>;

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
