#[derive(Clone, Copy, Debug)]
pub struct RemoveModules<'call> {
    pub module_key: &'call [&'call str],
}

impl crate::Operation for RemoveModules<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/wiki/rest/atlassian-connect/1/app/module/dynamic";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts_joined("moduleKey", Some(self.module_key))?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
