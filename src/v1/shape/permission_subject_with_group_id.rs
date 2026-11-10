#[derive(Clone, Copy, Debug)]
pub struct PermissionSubjectWithGroupID;

impl PermissionSubjectWithGroupID {
    pub const IDENTIFIER: crate::Field<Self, crate::Text> = crate::Field::new("identifier");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
