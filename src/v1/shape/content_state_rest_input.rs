#[derive(Clone, Copy, Debug)]
pub struct ContentStateRestInput;

impl ContentStateRestInput {
    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
