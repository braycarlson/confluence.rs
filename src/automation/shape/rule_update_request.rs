#[derive(Clone, Copy, Debug)]
pub struct RuleUpdateRequest;

impl RuleUpdateRequest {
    pub const CONNECTIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::RuleUpdateRequestConnections>>,
    > = crate::Field::new("connections");

    pub const RULE: crate::Field<Self, crate::Object<super::RuleUpdateRequestRule>> =
        crate::Field::new("rule");
}
