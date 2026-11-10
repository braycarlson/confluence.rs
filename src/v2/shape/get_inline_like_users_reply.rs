#[derive(Clone, Copy, Debug)]
pub struct GetInlineLikeUsersReply;

impl GetInlineLikeUsersReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::Like>>> =
        crate::Field::new("results");
}
