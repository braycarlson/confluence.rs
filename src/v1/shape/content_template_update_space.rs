#[derive(Clone, Copy, Debug)]
pub struct ContentTemplateUpdateSpace;

impl ContentTemplateUpdateSpace {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
}
