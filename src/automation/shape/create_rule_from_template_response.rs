#[derive(Clone, Copy, Debug)]
pub struct CreateRuleFromTemplateResponse;

impl CreateRuleFromTemplateResponse {
    pub const RULE_ID: crate::Field<Self, crate::Number> = crate::Field::new("ruleId");
    pub const RULE_UUID: crate::Field<Self, crate::Text> = crate::Field::new("ruleUuid");
}
