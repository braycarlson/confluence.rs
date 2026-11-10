#[derive(Clone, Copy, Debug)]
pub struct ContentRestrictionRestrictions;

impl ContentRestrictionRestrictions {
    pub const EXPANDABLE: crate::Field<
        Self,
        crate::Object<super::ContentRestrictionRestrictionsExpandable>,
    > = crate::Field::new("_expandable");

    pub const GROUP: crate::Field<Self, crate::Object<super::GroupArray>> =
        crate::Field::new("group");

    pub const USER: crate::Field<Self, crate::Object<super::UserArray>> = crate::Field::new("user");
}
