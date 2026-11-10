#[derive(Clone, Copy, Debug)]
pub struct BulkAssignRolesRequest;

impl BulkAssignRolesRequest {
    pub const ASSIGNMENTS: crate::Field<
        Self,
        crate::List<crate::Object<super::BulkTransitionRoleAssignment>>,
    > = crate::Field::new("assignments");

    pub const SPACE_SELECTION: crate::Field<
        Self,
        crate::Object<super::BulkTransitionSpaceSelection>,
    > = crate::Field::new("spaceSelection");
}
