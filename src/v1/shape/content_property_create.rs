#[derive(Clone, Copy, Debug)]
pub struct ContentPropertyCreate;

impl ContentPropertyCreate {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
