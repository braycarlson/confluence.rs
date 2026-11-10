#[derive(Clone, Copy, Debug)]
pub struct LongTaskStatusName;

impl LongTaskStatusName {
    pub const ARGUMENTS: crate::Field<Self, crate::List<crate::Any>> = crate::Field::new("args");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
}
