#[derive(Clone, Copy, Debug)]
pub struct SpaceDescriptionRepresentations;

impl SpaceDescriptionRepresentations {
    pub const EXPANDABLE: crate::Field<
        Self,
        crate::Object<super::SpaceDescriptionRepresentationsExpandable>,
    > = crate::Field::new("_expandable");

    pub const PLAIN: crate::Field<Self, crate::Object<super::SpaceDescription>> =
        crate::Field::new("plain");

    pub const VIEW: crate::Field<Self, crate::Object<super::SpaceDescription>> =
        crate::Field::new("view");
}
