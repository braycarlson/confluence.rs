#[derive(Clone, Copy, Debug)]
pub struct SpacePermissionCreateSubjectsUser;

impl SpacePermissionCreateSubjectsUser {
    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::User>>> =
        crate::Field::new("results");

    pub const SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("size");
}
