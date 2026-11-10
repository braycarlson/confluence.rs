#[derive(Clone, Copy, Debug)]
pub struct ClassificationLevel;

impl ClassificationLevel {
    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const GUIDELINE: crate::Field<Self, crate::Text> = crate::Field::new("guideline");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const ORDER: crate::Field<Self, crate::Number> = crate::Field::new("order");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
}
