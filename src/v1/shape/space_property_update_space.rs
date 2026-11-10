#[derive(Clone, Copy, Debug)]
pub struct SpacePropertyUpdateSpace;

impl SpacePropertyUpdateSpace {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
}
