#[derive(Clone, Copy, Debug)]
pub struct SpacePermissionCreateSubjects;

impl SpacePermissionCreateSubjects {
    pub const GROUP: crate::Field<Self, crate::Object<super::SpacePermissionCreateSubjectsGroup>> =
        crate::Field::new("group");

    pub const USER: crate::Field<Self, crate::Object<super::SpacePermissionCreateSubjectsUser>> =
        crate::Field::new("user");
}
