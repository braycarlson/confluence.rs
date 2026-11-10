#[derive(Clone, Copy, Debug)]
pub struct SpacePropertyUpdate;

impl SpacePropertyUpdate {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");

    pub const SPACE: crate::Field<Self, crate::Object<super::SpacePropertyUpdateSpace>> =
        crate::Field::new("space");

    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");

    pub const VERSION: crate::Field<Self, crate::Object<super::Version>> =
        crate::Field::new("version");
}
