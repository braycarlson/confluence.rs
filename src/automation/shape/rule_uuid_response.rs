#[derive(Clone, Copy, Debug)]
pub struct RuleUUIDResponse;

impl RuleUUIDResponse {
    pub const RULE_UUID: crate::Field<Self, crate::Text> = crate::Field::new("ruleUuid");
}
