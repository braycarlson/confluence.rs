#[derive(Clone, Copy, Debug)]
pub struct ThemeUpdate;

impl ThemeUpdate {
    pub const THEME_KEY: crate::Field<Self, crate::Text> = crate::Field::new("themeKey");
}
