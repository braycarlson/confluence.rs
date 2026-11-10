#[derive(Clone, Copy, Debug)]
pub struct DatabasePageLinkValue;

impl DatabasePageLinkValue {
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const VALUE: crate::Field<Self, crate::List<crate::Object<super::DatabasePageRef>>> =
        crate::Field::new("value");
}
