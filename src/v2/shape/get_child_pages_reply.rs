#[derive(Clone, Copy, Debug)]
pub struct GetChildPagesReply;

impl GetChildPagesReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::ChildPage>>> =
        crate::Field::new("results");
}
