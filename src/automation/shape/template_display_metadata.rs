#[derive(Clone, Copy, Debug)]
pub struct TemplateDisplayMetadata;

impl TemplateDisplayMetadata {
    pub const ACTION_ICONS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("actionIcons");

    pub const TRIGGER_ICONS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("triggerIcons");
}
