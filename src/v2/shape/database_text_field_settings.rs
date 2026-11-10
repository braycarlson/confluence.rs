#[derive(Clone, Copy, Debug)]
pub struct DatabaseTextFieldSettings;

impl DatabaseTextFieldSettings {
    pub const RICH_TEXT: crate::Field<Self, crate::Boolean> = crate::Field::new("richText");
}
