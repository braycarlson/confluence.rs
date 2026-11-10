#[derive(Clone, Copy, Debug)]
pub struct ContentPermissionError;

impl ContentPermissionError {
    pub const CODE: crate::Field<Self, crate::Text> = crate::Field::new("code");
    pub const DETAIL: crate::Field<Self, crate::Text> = crate::Field::new("detail");
    pub const STATUS: crate::Field<Self, crate::Integer> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
}
