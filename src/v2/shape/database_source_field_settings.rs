#[derive(Clone, Copy, Debug)]
pub struct DatabaseSourceFieldSettings;

impl DatabaseSourceFieldSettings {
    pub const SOURCE_FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("sourceFieldId");
}
