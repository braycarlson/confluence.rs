#[derive(Clone, Copy, Debug)]
pub struct ContentCreateSpaceDescription;

impl ContentCreateSpaceDescription {
    pub const EXPANDABLE: crate::Field<
        Self,
        crate::Object<super::ContentCreateSpaceDescriptionExpandable>,
    > = crate::Field::new("_expandable");

    pub const PLAIN: crate::Field<Self, crate::Object<super::SpaceDescription>> =
        crate::Field::new("plain");

    pub const VIEW: crate::Field<Self, crate::Object<super::SpaceDescription>> =
        crate::Field::new("view");
}
