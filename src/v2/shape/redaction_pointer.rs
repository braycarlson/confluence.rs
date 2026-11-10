#[derive(Clone, Copy, Debug)]
pub struct RedactionPointer;

impl RedactionPointer {
    pub const FROM: crate::Field<Self, crate::Integer> = crate::Field::new("from");
    pub const POINTER: crate::Field<Self, crate::Text> = crate::Field::new("pointer");
    pub const REASON: crate::Field<Self, crate::Text> = crate::Field::new("reason");
    pub const TO_VALUE: crate::Field<Self, crate::Integer> = crate::Field::new("to");
}
