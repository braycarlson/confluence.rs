#[derive(Clone, Copy, Debug)]
pub struct DatabaseSelectFieldSettings;

impl DatabaseSelectFieldSettings {
    pub const OPTIONS: crate::Field<Self, crate::List<crate::Object<super::DatabaseSelectOption>>> =
        crate::Field::new("options");
}
