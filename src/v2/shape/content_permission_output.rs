#[derive(Clone, Copy, Debug)]
pub struct ContentPermissionOutput;

impl ContentPermissionOutput {
    pub const CREATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("createdAt");
    pub const CREATED_BY: crate::Field<Self, crate::Text> = crate::Field::new("createdBy");
    pub const CREATED_BY_AAID: crate::Field<Self, crate::Text> = crate::Field::new("createdByAaid");
    pub const OPERATION: crate::Field<Self, crate::Text> = crate::Field::new("operation");

    pub const PRINCIPAL: crate::Field<Self, crate::Object<super::ContentPermissionPrincipal>> =
        crate::Field::new("principal");
}
