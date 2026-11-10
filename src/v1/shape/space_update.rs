#[derive(Clone, Copy, Debug)]
pub struct SpaceUpdate;

impl SpaceUpdate {
    pub const DESCRIPTION: crate::Field<Self, crate::Object<super::SpaceDescriptionCreate>> =
        crate::Field::new("description");

    pub const HOMEPAGE: crate::Field<Self, crate::Any> = crate::Field::new("homepage");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
