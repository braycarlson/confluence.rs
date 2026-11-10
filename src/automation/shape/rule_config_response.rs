#[derive(Clone, Copy, Debug)]
pub struct RuleConfigResponse;

impl RuleConfigResponse {
    pub const CONNECTIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::RuleConfigResponseConnections>>,
    > = crate::Field::new("connections");

    pub const RULE: crate::Field<Self, crate::Object<super::RuleConfigResponseRule>> =
        crate::Field::new("rule");
}
