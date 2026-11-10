#[derive(Clone, Copy, Debug)]
pub struct GetTemplateByTemplateID<'call> {
    pub template_id: &'call str,
}

impl crate::Operation for GetTemplateByTemplateID<'_> {
    const BASE: crate::Base = crate::Base::Automation;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/v1/template/{templateId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::Template>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("templateId", self.template_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
