#[derive(Clone, Copy, Debug)]
pub struct SpacePermissionAssignment;

impl SpacePermissionAssignment {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const OPERATION: crate::Field<
        Self,
        crate::Object<super::SpacePermissionAssignmentOperation>,
    > = crate::Field::new("operation");

    pub const PRINCIPAL: crate::Field<
        Self,
        crate::Object<super::SpacePermissionAssignmentPrincipal>,
    > = crate::Field::new("principal");
}
