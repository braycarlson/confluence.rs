#[derive(Clone, Copy, Debug)]
pub struct GetRuleByUUID<'call> {
    pub rule_uuid: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub redact_sensitive_fields: Option<bool>,
}

impl crate::Operation for GetRuleByUUID<'_> {
    const BASE: crate::Base = crate::Base::Automation;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/v1/rule/{ruleUuid}";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::RuleConfigResponse>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("redactSensitiveFields", options.redact_sensitive_fields);
        request.path_text("ruleUuid", self.rule_uuid)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
