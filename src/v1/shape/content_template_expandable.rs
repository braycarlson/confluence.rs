#[derive(Clone, Copy, Debug)]
pub struct ContentTemplateExpandable;

impl ContentTemplateExpandable {
    pub const BODY: crate::Field<Self, crate::Text> = crate::Field::new("body");
}
