#[derive(Clone, Copy, Debug)]
pub struct GetAttachmentVersionsReply;

impl GetAttachmentVersionsReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::AttachmentVersion>>> =
        crate::Field::new("results");
}
