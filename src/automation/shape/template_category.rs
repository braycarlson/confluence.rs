#[derive(Clone, Copy, Debug)]
pub struct TemplateCategory;

impl TemplateCategory {
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
}
