#[derive(Clone, Copy, Debug)]
pub struct ContentRestrictionRestrictionsExpandable;

impl ContentRestrictionRestrictionsExpandable {
    pub const GROUP: crate::Field<Self, crate::Text> = crate::Field::new("group");
    pub const USER: crate::Field<Self, crate::Text> = crate::Field::new("user");
}
