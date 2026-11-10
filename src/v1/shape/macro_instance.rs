#[derive(Clone, Copy, Debug)]
pub struct MacroInstance;

impl MacroInstance {
    pub const BODY: crate::Field<Self, crate::Text> = crate::Field::new("body");
    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const PARAMETERS: crate::Field<Self, crate::Any> = crate::Field::new("parameters");
}
