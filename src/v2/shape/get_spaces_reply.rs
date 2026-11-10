#[derive(Clone, Copy, Debug)]
pub struct GetSpacesReply;

impl GetSpacesReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::SpaceBulk>>> =
        crate::Field::new("results");
}
