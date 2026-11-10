#[derive(Clone, Copy, Debug)]
pub struct BulkTransitionDecodedPermission;

impl BulkTransitionDecodedPermission {
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
