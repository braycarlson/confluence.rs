#[derive(Clone, Copy, Debug)]
pub struct GetLabelsReply;

impl GetLabelsReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::Label>>> =
        crate::Field::new("results");
}
