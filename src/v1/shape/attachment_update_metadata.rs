#[derive(Clone, Copy, Debug)]
pub struct AttachmentUpdateMetadata;

impl AttachmentUpdateMetadata {
    pub const COMMENT: crate::Field<Self, crate::Text> = crate::Field::new("comment");
    pub const MEDIA_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("mediaType");
}
