#[derive(Clone, Copy, Debug)]
pub struct OptionalFieldMeta;

impl OptionalFieldMeta {
    pub const COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("count");
    pub const CURSOR: crate::Field<Self, crate::Text> = crate::Field::new("cursor");
    pub const HAS_MORE: crate::Field<Self, crate::Boolean> = crate::Field::new("hasMore");
}
