#[derive(Clone, Copy, Debug)]
pub struct GetDatabaseDescendantsReply;

impl GetDatabaseDescendantsReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::DescendantsResponse>>> =
        crate::Field::new("results");
}
