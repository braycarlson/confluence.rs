#[derive(Clone, Copy, Debug)]
pub struct DeleteDisabledRule<'call> {
    pub rule_uuid: &'call str,
}

impl crate::Operation for DeleteDisabledRule<'_> {
    const BASE: crate::Base = crate::Base::Automation;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/v1/rule/{ruleUuid}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("ruleUuid", self.rule_uuid)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
