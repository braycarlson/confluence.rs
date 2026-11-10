#[derive(Clone, Copy, Debug)]
pub struct ContentRestrictionUpdateRestrictions;

impl ContentRestrictionUpdateRestrictions {
    pub const GROUP: crate::Field<Self, crate::Any> = crate::Field::new("group");
    pub const USER: crate::Field<Self, crate::Any> = crate::Field::new("user");
}
