#[derive(Clone, Copy, Debug)]
pub struct BulkTransitionRoleAssignment;

impl BulkTransitionRoleAssignment {
    pub const PERMISSION_COMBINATION_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("permissionCombinationId");

    pub const PRINCIPAL_TYPE_ASSIGNMENTS: crate::Field<
        Self,
        crate::List<crate::Object<super::BulkTransitionPrincipalTypeAssignment>>,
    > = crate::Field::new("principalTypeAssignments");
}
