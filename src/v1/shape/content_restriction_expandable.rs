#[derive(Clone, Copy, Debug)]
pub struct ContentRestrictionExpandable;

impl ContentRestrictionExpandable {
    pub const CONTENT: crate::Field<Self, crate::Text> = crate::Field::new("content");
    pub const RESTRICTIONS: crate::Field<Self, crate::Text> = crate::Field::new("restrictions");
}
