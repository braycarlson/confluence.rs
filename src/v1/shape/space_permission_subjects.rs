#[derive(Clone, Copy, Debug)]
pub struct SpacePermissionSubjects;

impl SpacePermissionSubjects {
    pub const EXPANDABLE: crate::Field<
        Self,
        crate::Object<super::SpacePermissionSubjectsExpandable>,
    > = crate::Field::new("_expandable");

    pub const GROUP: crate::Field<Self, crate::Object<super::SpacePermissionSubjectsGroup>> =
        crate::Field::new("group");

    pub const USER: crate::Field<Self, crate::Object<super::SpacePermissionSubjectsUser>> =
        crate::Field::new("user");
}
