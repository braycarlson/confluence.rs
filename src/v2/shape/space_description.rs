#[derive(Clone, Copy, Debug)]
pub struct SpaceDescription;

impl SpaceDescription {
    pub const PLAIN: crate::Field<Self, crate::Object<super::BodyType>> =
        crate::Field::new("plain");

    pub const VIEW: crate::Field<Self, crate::Object<super::BodyType>> = crate::Field::new("view");
}
