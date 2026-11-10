#[derive(Clone, Copy, Debug)]
pub struct ContentPropertyCreateNoKey;

impl ContentPropertyCreateNoKey {
    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
