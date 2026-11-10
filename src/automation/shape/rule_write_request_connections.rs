#[derive(Clone, Copy, Debug)]
pub struct RuleWriteRequestConnections;

impl RuleWriteRequestConnections {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const AUTHENTICATION_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("authType");

    pub const CONNECTION_TARGET_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("connectionTargetKey");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const TARGET_CONFIG_JSON: crate::Field<Self, crate::Text> =
        crate::Field::new("targetConfigJson");
}
