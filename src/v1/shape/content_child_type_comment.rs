#[derive(Clone, Copy, Debug)]
pub struct ContentChildTypeComment;

impl ContentChildTypeComment {
    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");
    pub const VALUE: crate::Field<Self, crate::Boolean> = crate::Field::new("value");
}
