#[derive(Clone, Copy, Debug)]
pub struct Operation;

impl Operation {
    pub const OPERATION: crate::Field<Self, crate::Text> = crate::Field::new("operation");
    pub const TARGET_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("targetType");
}
