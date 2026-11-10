#[derive(Clone, Copy, Debug)]
pub struct SpaceCreate;

impl SpaceCreate {
    pub const ALIAS: crate::Field<Self, crate::Text> = crate::Field::new("alias");

    pub const DESCRIPTION: crate::Field<Self, crate::Object<super::SpaceDescriptionCreate>> =
        crate::Field::new("description");

    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PERMISSIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::SpacePermissionCreate>>,
    > = crate::Field::new("permissions");
}
