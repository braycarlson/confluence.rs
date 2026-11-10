#[derive(Clone, Copy, Debug)]
pub struct DatabasePageDetailsFieldSettings;

impl DatabasePageDetailsFieldSettings {
    pub const PAGE_PROPERTY_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("pagePropertyKey");

    pub const SOURCE_FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("sourceFieldId");
}
