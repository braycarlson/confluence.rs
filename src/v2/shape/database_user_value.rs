#[derive(Clone, Copy, Debug)]
pub struct DatabaseUserValue;

impl DatabaseUserValue {
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const VALUE: crate::Field<Self, crate::List<crate::Object<super::DatabaseUserRef>>> =
        crate::Field::new("value");
}
