#[derive(Clone, Copy, Debug)]
pub struct SpacePermissionCustomContentOperations;

impl SpacePermissionCustomContentOperations {
    pub const ACCESS: crate::Field<Self, crate::Boolean> = crate::Field::new("access");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const TARGET: crate::Field<Self, crate::Text> = crate::Field::new("target");
}
