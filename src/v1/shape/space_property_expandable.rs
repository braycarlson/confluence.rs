#[derive(Clone, Copy, Debug)]
pub struct SpacePropertyExpandable;

impl SpacePropertyExpandable {
    pub const SPACE: crate::Field<Self, crate::Text> = crate::Field::new("space");
    pub const VERSION: crate::Field<Self, crate::Text> = crate::Field::new("version");
}
