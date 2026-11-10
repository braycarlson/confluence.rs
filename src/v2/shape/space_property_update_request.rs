#[derive(Clone, Copy, Debug)]
pub struct SpacePropertyUpdateRequest;

impl SpacePropertyUpdateRequest {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");

    pub const VERSION: crate::Field<Self, crate::Object<super::SpacePropertyUpdateRequestVersion>> =
        crate::Field::new("version");
}
