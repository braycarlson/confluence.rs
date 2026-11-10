#[derive(Clone, Copy, Debug)]
pub struct ContentPermissionRequest;

impl ContentPermissionRequest {
    pub const OPERATION: crate::Field<Self, crate::Text> = crate::Field::new("operation");

    pub const SUBJECT: crate::Field<Self, crate::Object<super::PermissionSubjectWithGroupID>> =
        crate::Field::new("subject");
}
