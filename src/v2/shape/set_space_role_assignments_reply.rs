#[derive(Clone, Copy, Debug)]
pub struct SetSpaceRoleAssignmentsReply;

impl SetSpaceRoleAssignmentsReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::SpaceRoleAssignment>>> =
        crate::Field::new("results");
}
