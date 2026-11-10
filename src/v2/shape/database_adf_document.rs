#[derive(Clone, Copy, Debug)]
pub struct DatabaseADFDocument;

impl DatabaseADFDocument {
    pub const CONTENT: crate::Field<Self, crate::List<crate::Any>> = crate::Field::new("content");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const VERSION: crate::Field<Self, crate::Integer> = crate::Field::new("version");
}
