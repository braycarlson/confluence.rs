#[derive(Clone, Copy, Debug)]
pub struct DatabaseHyperlinkValue;

impl DatabaseHyperlinkValue {
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const VALUE: crate::Field<Self, crate::List<crate::Object<super::DatabaseHyperlinkItem>>> =
        crate::Field::new("value");
}
