#[derive(Clone, Copy, Debug)]
pub struct GetAnonymousUser;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options [&'options str]>,
}

impl crate::Operation for GetAnonymousUser {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/rest/api/user/anonymous";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::UserAnonymous>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts_joined("expand", options.expand)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
