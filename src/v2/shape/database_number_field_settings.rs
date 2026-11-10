#[derive(Clone, Copy, Debug)]
pub struct DatabaseNumberFieldSettings;

impl DatabaseNumberFieldSettings {
    pub const FORMAT: crate::Field<Self, crate::Object<super::DatabaseNumberFormat>> =
        crate::Field::new("format");
}
