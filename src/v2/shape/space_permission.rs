#[derive(Clone, Copy, Debug)]
pub struct SpacePermission;

impl SpacePermission {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const REQUIRED_PERMISSION_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("requiredPermissionIds");
}
