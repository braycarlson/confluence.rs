#[derive(Clone, Copy, Debug)]
pub struct DatabaseCheckboxValue;

impl DatabaseCheckboxValue {
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const VALUE: crate::Field<Self, crate::Boolean> = crate::Field::new("value");
}
