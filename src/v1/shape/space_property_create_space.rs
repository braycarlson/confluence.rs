#[derive(Clone, Copy, Debug)]
pub struct SpacePropertyCreateSpace;

impl SpacePropertyCreateSpace {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
}
