#[derive(Clone, Copy, Debug)]
pub struct ContentTemplateCreateSpace;

impl ContentTemplateCreateSpace {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
}
