#[derive(Clone, Copy, Debug)]
pub struct BulkTransitionSpaceTarget;

impl BulkTransitionSpaceTarget {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
}
