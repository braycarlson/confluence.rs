#[derive(Clone, Copy, Debug)]
pub struct SpaceProperty;

impl SpaceProperty {
    pub const EXPANDABLE: crate::Field<Self, crate::Object<super::SpacePropertyExpandable>> =
        crate::Field::new("_expandable");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");
    pub const SPACE: crate::Field<Self, crate::Object<super::Space>> = crate::Field::new("space");
    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");

    pub const VERSION: crate::Field<Self, crate::Object<super::Version>> =
        crate::Field::new("version");
}
