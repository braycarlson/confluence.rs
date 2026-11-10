#[derive(Clone, Copy, Debug)]
pub struct DatabaseEntryLinkField;

impl DatabaseEntryLinkField {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SETTINGS: crate::Field<Self, crate::Any> = crate::Field::new("settings");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
