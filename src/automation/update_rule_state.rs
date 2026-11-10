#[derive(Clone, Copy, Debug)]
pub struct UpdateRuleState<'call> {
    pub rule_uuid: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub body: Option<&'options serde_json::Value>,
}

impl crate::Operation for UpdateRuleState<'_> {
    const BASE: crate::Base = crate::Base::Automation;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/rest/v1/rule/{ruleUuid}/state";

    type Options<'options> = Options<'options>;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("ruleUuid", self.rule_uuid)?;

        if let Some(body) = options.body {
            request.body_json(body)?;
        }

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
