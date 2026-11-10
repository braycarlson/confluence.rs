#[derive(Clone, Copy, Debug)]
pub struct SearchTemplatesWithCursorRequest;

impl SearchTemplatesWithCursorRequest {
    pub const CURSOR: crate::Field<Self, crate::Text> = crate::Field::new("cursor");
    pub const LIMIT: crate::Field<Self, crate::Number> = crate::Field::new("limit");
}
