#[derive(Clone, Copy, Debug)]
pub struct SpacePermissionSubjectsUser;

impl SpacePermissionSubjectsUser {
    pub const LIMIT: crate::Field<Self, crate::Integer> = crate::Field::new("limit");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::User>>> =
        crate::Field::new("results");

    pub const SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("size");
    pub const START: crate::Field<Self, crate::Integer> = crate::Field::new("start");
}
