#[derive(Clone, Copy, Debug)]
pub struct ContentBodyCreate;

impl ContentBodyCreate {
    pub const REPRESENTATION: crate::Field<Self, crate::Text> = crate::Field::new("representation");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
