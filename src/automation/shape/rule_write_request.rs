#[derive(Clone, Copy, Debug)]
pub struct RuleWriteRequest;

impl RuleWriteRequest {
    pub const CONNECTIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::RuleWriteRequestConnections>>,
    > = crate::Field::new("connections");

    pub const RULE: crate::Field<Self, crate::Object<super::RuleWriteRequestRule>> =
        crate::Field::new("rule");
}
