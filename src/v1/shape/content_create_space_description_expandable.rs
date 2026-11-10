#[derive(Clone, Copy, Debug)]
pub struct ContentCreateSpaceDescriptionExpandable;

impl ContentCreateSpaceDescriptionExpandable {
    pub const PLAIN: crate::Field<Self, crate::Text> = crate::Field::new("plain");
    pub const VIEW: crate::Field<Self, crate::Text> = crate::Field::new("view");
}
