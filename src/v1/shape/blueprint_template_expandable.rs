#[derive(Clone, Copy, Debug)]
pub struct BlueprintTemplateExpandable;

impl BlueprintTemplateExpandable {
    pub const BODY: crate::Field<Self, crate::Text> = crate::Field::new("body");
}
