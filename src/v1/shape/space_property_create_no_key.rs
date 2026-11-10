#[derive(Clone, Copy, Debug)]
pub struct SpacePropertyCreateNoKey;

impl SpacePropertyCreateNoKey {
    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
