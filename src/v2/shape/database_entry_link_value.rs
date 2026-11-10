#[derive(Clone, Copy, Debug)]
pub struct DatabaseEntryLinkValue;

impl DatabaseEntryLinkValue {
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const VALUE: crate::Field<Self, crate::List<crate::Object<super::DatabaseEntryRef>>> =
        crate::Field::new("value");
}
