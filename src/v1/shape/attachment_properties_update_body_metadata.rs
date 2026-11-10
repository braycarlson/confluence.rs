#[derive(Clone, Copy, Debug)]
pub struct AttachmentPropertiesUpdateBodyMetadata;

impl AttachmentPropertiesUpdateBodyMetadata {
    pub const MEDIA_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("mediaType");
}
