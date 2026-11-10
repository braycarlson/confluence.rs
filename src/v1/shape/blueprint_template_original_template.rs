#[derive(Clone, Copy, Debug)]
pub struct BlueprintTemplateOriginalTemplate;

impl BlueprintTemplateOriginalTemplate {
    pub const MODULE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("moduleKey");
    pub const PLUGIN_KEY: crate::Field<Self, crate::Text> = crate::Field::new("pluginKey");
}
