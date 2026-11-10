#[derive(Clone, Copy, Debug)]
pub struct RuleConfigResponseConnections;

impl RuleConfigResponseConnections {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const AUTHENTICATION_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("authType");

    pub const CONNECTION_TARGET_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("connectionTargetKey");

    pub const CONTAINER: crate::Field<
        Self,
        crate::Object<super::RuleConfigResponseConnectionsContainer>,
    > = crate::Field::new("container");

    pub const CREATED_AT: crate::Field<Self, crate::Number> = crate::Field::new("createdAt");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const TARGET_CONFIG_JSON: crate::Field<Self, crate::Text> =
        crate::Field::new("targetConfigJson");

    pub const UPDATED_AT: crate::Field<Self, crate::Number> = crate::Field::new("updatedAt");
}
