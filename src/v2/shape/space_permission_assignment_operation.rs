#[derive(Clone, Copy, Debug)]
pub struct SpacePermissionAssignmentOperation;

impl SpacePermissionAssignmentOperation {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const TARGET_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("targetType");
}
