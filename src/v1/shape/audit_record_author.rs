#[derive(Clone, Copy, Debug)]
pub struct AuditRecordAuthor;

impl AuditRecordAuthor {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const ACCOUNT_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("accountType");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");

    pub const EXTERNAL_COLLABORATOR: crate::Field<Self, crate::Boolean> =
        crate::Field::new("externalCollaborator");

    pub const IS_EXTERNAL_COLLABORATOR: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isExternalCollaborator");

    pub const IS_GUEST: crate::Field<Self, crate::Boolean> = crate::Field::new("isGuest");

    pub const OPERATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::OperationCheckResult>>,
    > = crate::Field::new("operations");

    pub const PUBLIC_NAME: crate::Field<Self, crate::Text> = crate::Field::new("publicName");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const USERNAME: crate::Field<Self, crate::Text> = crate::Field::new("username");
    pub const USER_KEY: crate::Field<Self, crate::Text> = crate::Field::new("userKey");
}
