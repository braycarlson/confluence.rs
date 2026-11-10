#[derive(Clone, Copy, Debug)]
pub struct SpacePermissionRequestOperation;

impl SpacePermissionRequestOperation {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const TARGET: crate::Field<Self, crate::Text> = crate::Field::new("target");
}
