#[derive(Clone, Copy, Debug)]
pub struct Error;

impl Error {
    pub const CODE: crate::Field<Self, crate::Text> = crate::Field::new("code");
    pub const DETAIL: crate::Field<Self, crate::Text> = crate::Field::new("detail");
    pub const FIELD: crate::Field<Self, crate::Text> = crate::Field::new("field");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const STATUS: crate::Field<Self, crate::Number> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
}
