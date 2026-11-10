#[derive(Clone, Copy, Debug)]
pub struct DatabaseDateFieldSettings;

impl DatabaseDateFieldSettings {
    pub const INCLUDES_TIME: crate::Field<Self, crate::Boolean> = crate::Field::new("includesTime");
}
