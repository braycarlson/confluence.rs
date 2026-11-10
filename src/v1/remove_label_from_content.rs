#[derive(Clone, Copy, Debug)]
pub struct RemoveLabelFromContent<'call> {
    pub id: &'call str,
    pub label: &'call str,
}

impl crate::Operation for RemoveLabelFromContent<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/wiki/rest/api/content/{id}/label/{label}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;
        request.path_text("label", self.label)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
