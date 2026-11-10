#[derive(Clone, Copy, Debug)]
pub struct LookAndFeelSettings;

impl LookAndFeelSettings {
    pub const CUSTOM: crate::Field<Self, crate::Object<super::LookAndFeel>> =
        crate::Field::new("custom");

    pub const GLOBAL: crate::Field<Self, crate::Object<super::LookAndFeel>> =
        crate::Field::new("global");

    pub const SELECTED: crate::Field<Self, crate::Text> = crate::Field::new("selected");

    pub const THEME: crate::Field<Self, crate::Object<super::LookAndFeel>> =
        crate::Field::new("theme");
}
