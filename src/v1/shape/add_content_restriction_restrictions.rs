#[derive(Clone, Copy, Debug)]
pub struct AddContentRestrictionRestrictions;

impl AddContentRestrictionRestrictions {
    pub const GROUP: crate::Field<
        Self,
        crate::List<crate::Object<super::AddContentRestrictionRestrictionsGroup>>,
    > = crate::Field::new("group");

    pub const USER: crate::Field<
        Self,
        crate::List<crate::Object<super::AddContentRestrictionRestrictionsUser>>,
    > = crate::Field::new("user");
}
