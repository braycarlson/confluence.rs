#[derive(Clone, Copy, Debug)]
pub struct BulkTransitionPrincipalTypeAssignment;

impl BulkTransitionPrincipalTypeAssignment {
    pub const PRINCIPAL_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("principalType");
    pub const REMOVE_ACCESS: crate::Field<Self, crate::Boolean> = crate::Field::new("removeAccess");
    pub const ROLE_ID: crate::Field<Self, crate::Text> = crate::Field::new("roleId");
}
