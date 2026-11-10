#[derive(Clone, Copy, Debug)]
pub struct DatabaseSelectValue;

impl DatabaseSelectValue {
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const VALUE: crate::Field<Self, crate::List<crate::Object<super::DatabaseSelectOption>>> =
        crate::Field::new("value");
}
