#[derive(Clone, Copy, Debug)]
pub struct ContentCreateAncestors;

impl ContentCreateAncestors {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
