#[derive(Clone, Copy, Debug)]
pub struct UpdateLookAndFeel<'call> {
    pub body: &'call serde_json::Value,
}

impl crate::Operation for UpdateLookAndFeel<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/wiki/rest/api/settings/lookandfeel";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::LookAndFeelSelection>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
