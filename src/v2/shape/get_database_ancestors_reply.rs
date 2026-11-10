#[derive(Clone, Copy, Debug)]
pub struct GetDatabaseAncestorsReply;

impl GetDatabaseAncestorsReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::Ancestor>>> =
        crate::Field::new("results");
}
