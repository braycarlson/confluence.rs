#[derive(Clone, Copy, Debug)]
pub struct GetCustomContentByTypeInSpaceReply;

impl GetCustomContentByTypeInSpaceReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::CustomContentBulk>>> =
        crate::Field::new("results");
}
