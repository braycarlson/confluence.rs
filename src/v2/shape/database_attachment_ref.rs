#[derive(Clone, Copy, Debug)]
pub struct DatabaseAttachmentRef;

impl DatabaseAttachmentRef {
    pub const MEDIA_ID: crate::Field<Self, crate::Text> = crate::Field::new("mediaId");
}
