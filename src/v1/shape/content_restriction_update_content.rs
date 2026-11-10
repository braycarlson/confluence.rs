#[derive(Clone, Copy, Debug)]
pub struct ContentRestrictionUpdateContent;

impl ContentRestrictionUpdateContent {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
