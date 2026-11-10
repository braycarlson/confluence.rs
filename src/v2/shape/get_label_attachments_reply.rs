#[derive(Clone, Copy, Debug)]
pub struct GetLabelAttachmentsReply;

impl GetLabelAttachmentsReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::AttachmentBulk>>> =
        crate::Field::new("results");
}
