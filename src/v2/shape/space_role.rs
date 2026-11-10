#[derive(Clone, Copy, Debug)]
pub struct SpaceRole;

impl SpaceRole {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const SPACE_PERMISSIONS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("spacePermissions");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
