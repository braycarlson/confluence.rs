#[derive(Clone, Copy, Debug)]
pub struct SpacePropertyCreate;

impl SpacePropertyCreate {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");

    pub const SPACE: crate::Field<Self, crate::Object<super::SpacePropertyCreateSpace>> =
        crate::Field::new("space");

    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
