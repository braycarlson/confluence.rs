#[derive(Clone, Copy, Debug)]
pub struct SpacePermissionRequest;

impl SpacePermissionRequest {
    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");

    pub const OPERATION: crate::Field<Self, crate::Object<super::SpacePermissionRequestOperation>> =
        crate::Field::new("operation");

    pub const SUBJECT: crate::Field<Self, crate::Object<super::PermissionSubject>> =
        crate::Field::new("subject");
}
