#[derive(Clone, Copy, Debug)]
pub struct GetAttachmentCommentsReply;

impl GetAttachmentCommentsReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<
        Self,
        crate::List<crate::Object<super::AttachmentCommentModel>>,
    > = crate::Field::new("results");
}
