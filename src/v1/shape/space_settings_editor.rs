#[derive(Clone, Copy, Debug)]
pub struct SpaceSettingsEditor;

impl SpaceSettingsEditor {
    pub const BLOGPOST: crate::Field<Self, crate::Text> = crate::Field::new("blogpost");
    pub const DEFAULT: crate::Field<Self, crate::Text> = crate::Field::new("default");
    pub const PAGE: crate::Field<Self, crate::Text> = crate::Field::new("page");
}
