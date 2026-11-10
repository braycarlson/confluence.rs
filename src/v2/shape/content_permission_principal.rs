#[derive(Clone, Copy, Debug)]
pub struct ContentPermissionPrincipal;

impl ContentPermissionPrincipal {
    pub const PRINCIPAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("principalId");
    pub const PRINCIPAL_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("principalType");
}
