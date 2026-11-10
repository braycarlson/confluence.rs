#[derive(Clone, Copy, Debug)]
pub struct SpacePermissionV2;

impl SpacePermissionV2 {
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");

    pub const OPERATION: crate::Field<Self, crate::Object<super::SpacePermissionV2Operation>> =
        crate::Field::new("operation");

    pub const SUBJECT: crate::Field<Self, crate::Object<super::PermissionSubject>> =
        crate::Field::new("subject");
}
