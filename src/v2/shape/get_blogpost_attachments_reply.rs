#[derive(Clone, Copy, Debug)]
pub struct GetBlogpostAttachmentsReply;

impl GetBlogpostAttachmentsReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::AttachmentBulk>>> =
        crate::Field::new("results");
}
