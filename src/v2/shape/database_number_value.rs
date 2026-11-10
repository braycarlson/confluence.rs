#[derive(Clone, Copy, Debug)]
pub struct DatabaseNumberValue;

impl DatabaseNumberValue {
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
