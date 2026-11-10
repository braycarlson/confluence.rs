#[derive(Clone, Copy, Debug)]
pub struct GetLabelPagesReply;

impl GetLabelPagesReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::PageBulk>>> =
        crate::Field::new("results");
}
