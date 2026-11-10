#[derive(Clone, Copy, Debug)]
pub struct GetPageVersionsReply;

impl GetPageVersionsReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::PageVersion>>> =
        crate::Field::new("results");
}
