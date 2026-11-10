#[derive(Clone, Copy, Debug)]
pub struct SpacePermissionCustomContent;

impl SpacePermissionCustomContent {
    pub const OPERATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::SpacePermissionCustomContentOperations>>,
    > = crate::Field::new("operations");

    pub const SUBJECT: crate::Field<Self, crate::Object<super::PermissionSubject>> =
        crate::Field::new("subject");
}
