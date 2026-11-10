#[derive(Clone, Copy, Debug)]
pub struct SpaceDescriptionCreate;

impl SpaceDescriptionCreate {
    pub const PLAIN: crate::Field<Self, crate::Object<super::SpaceDescriptionCreatePlain>> =
        crate::Field::new("plain");
}
