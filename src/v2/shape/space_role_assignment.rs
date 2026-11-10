#[derive(Clone, Copy, Debug)]
pub struct SpaceRoleAssignment;

impl SpaceRoleAssignment {
    pub const PRINCIPAL: crate::Field<Self, crate::Object<super::Principal>> =
        crate::Field::new("principal");

    pub const ROLE_ID: crate::Field<Self, crate::Text> = crate::Field::new("roleId");
}
