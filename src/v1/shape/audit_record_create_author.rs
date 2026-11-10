#[derive(Clone, Copy, Debug)]
pub struct AuditRecordCreateAuthor;

impl AuditRecordCreateAuthor {
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");

    pub const OPERATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::OperationCheckResult>>,
    > = crate::Field::new("operations");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const USERNAME: crate::Field<Self, crate::Text> = crate::Field::new("username");
    pub const USER_KEY: crate::Field<Self, crate::Text> = crate::Field::new("userKey");
}
