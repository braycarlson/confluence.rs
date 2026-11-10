#[derive(Clone, Copy, Debug)]
pub struct DatabaseAttachmentValue;

impl DatabaseAttachmentValue {
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const VALUE: crate::Field<Self, crate::List<crate::Object<super::DatabaseAttachmentRef>>> =
        crate::Field::new("value");
}
