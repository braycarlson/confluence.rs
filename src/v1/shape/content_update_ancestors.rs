#[derive(Clone, Copy, Debug)]
pub struct ContentUpdateAncestors;

impl ContentUpdateAncestors {
    pub const ID: crate::Field<Self, crate::Any> = crate::Field::new("id");
}
