#[derive(Clone, Copy, Debug)]
pub struct AttachmentLinks;

impl AttachmentLinks {
    pub const DOWNLOAD: crate::Field<Self, crate::Text> = crate::Field::new("download");
    pub const WEBUI: crate::Field<Self, crate::Text> = crate::Field::new("webui");
}
