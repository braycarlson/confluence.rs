#[derive(Clone, Copy, Debug)]
pub struct CreateRuleFromTemplateRequest;

impl CreateRuleFromTemplateRequest {
    pub const PARAMETERS: crate::Field<
        Self,
        crate::Map<crate::Object<super::CreateRuleFromTemplateParameterValue>>,
    > = crate::Field::new("parameters");

    pub const RULE_HOME: crate::Field<Self, crate::Text> = crate::Field::new("ruleHome");
    pub const STATE: crate::Field<Self, crate::Text> = crate::Field::new("state");
    pub const TEMPLATE_ID: crate::Field<Self, crate::Text> = crate::Field::new("templateId");
}
