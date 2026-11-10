#[derive(Clone, Copy, Debug)]
pub struct ContentRestrictionGroupInput;

impl ContentRestrictionGroupInput {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
