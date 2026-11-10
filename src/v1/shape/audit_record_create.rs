#[derive(Clone, Copy, Debug)]
pub struct AuditRecordCreate;

impl AuditRecordCreate {
    pub const AFFECTED_OBJECT: crate::Field<Self, crate::Object<super::AffectedObject>> =
        crate::Field::new("affectedObject");

    pub const ASSOCIATED_OBJECTS: crate::Field<
        Self,
        crate::List<crate::Object<super::AffectedObject>>,
    > = crate::Field::new("associatedObjects");

    pub const AUTHOR: crate::Field<Self, crate::Object<super::AuditRecordCreateAuthor>> =
        crate::Field::new("author");

    pub const CATEGORY: crate::Field<Self, crate::Text> = crate::Field::new("category");

    pub const CHANGED_VALUES: crate::Field<Self, crate::List<crate::Object<super::ChangedValue>>> =
        crate::Field::new("changedValues");

    pub const CREATION_DATE: crate::Field<Self, crate::Integer> = crate::Field::new("creationDate");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const REMOTE_ADDRESS: crate::Field<Self, crate::Text> = crate::Field::new("remoteAddress");
    pub const SUMMARY: crate::Field<Self, crate::Text> = crate::Field::new("summary");
    pub const SYS_ADMIN: crate::Field<Self, crate::Boolean> = crate::Field::new("sysAdmin");
}
