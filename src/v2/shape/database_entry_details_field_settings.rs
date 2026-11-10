#[derive(Clone, Copy, Debug)]
pub struct DatabaseEntryDetailsFieldSettings;

impl DatabaseEntryDetailsFieldSettings {
    pub const LOOKUP_FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("lookupFieldId");
    pub const SOURCE_FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("sourceFieldId");
}
