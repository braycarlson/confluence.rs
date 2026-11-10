#[derive(Clone, Copy, Debug)]
pub struct GetAvailableSpacePermissionsReply;

impl GetAvailableSpacePermissionsReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::SpacePermission>>> =
        crate::Field::new("results");
}
