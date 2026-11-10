#[derive(Clone, Copy, Debug)]
pub struct Principal;

impl Principal {
    pub const PRINCIPAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("principalId");
    pub const PRINCIPAL_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("principalType");
}
