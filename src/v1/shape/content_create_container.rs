#[derive(Clone, Copy, Debug)]
pub struct ContentCreateContainer;

impl ContentCreateContainer {
    pub const ID: crate::Field<Self, crate::Any> = crate::Field::new("id");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
