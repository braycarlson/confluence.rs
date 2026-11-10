#[derive(Clone, Copy, Debug)]
pub struct GetAttachmentLabelsReply;

impl GetAttachmentLabelsReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::Label>>> =
        crate::Field::new("results");
}
