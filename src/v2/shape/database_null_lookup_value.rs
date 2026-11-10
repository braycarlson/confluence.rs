#[derive(Clone, Copy, Debug)]
pub struct DatabaseNullLookupValue;

impl DatabaseNullLookupValue {
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
