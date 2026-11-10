#[derive(Clone, Copy, Debug)]
pub struct SpacePermissionV2Operation;

impl SpacePermissionV2Operation {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const TARGET: crate::Field<Self, crate::Text> = crate::Field::new("target");
}
