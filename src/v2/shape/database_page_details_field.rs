#[derive(Clone, Copy, Debug)]
pub struct DatabasePageDetailsField;

impl DatabasePageDetailsField {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const SETTINGS: crate::Field<Self, crate::Object<super::DatabasePageDetailsFieldSettings>> =
        crate::Field::new("settings");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
