#[derive(Clone, Copy, Debug)]
pub struct ThemeNoLinks;

impl ThemeNoLinks {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ICON: crate::Field<Self, crate::Object<super::Icon>> = crate::Field::new("icon");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const THEME_KEY: crate::Field<Self, crate::Text> = crate::Field::new("themeKey");
}
