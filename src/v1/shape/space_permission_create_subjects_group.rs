#[derive(Clone, Copy, Debug)]
pub struct SpacePermissionCreateSubjectsGroup;

impl SpacePermissionCreateSubjectsGroup {
    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::GroupCreate>>> =
        crate::Field::new("results");

    pub const SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("size");
}
