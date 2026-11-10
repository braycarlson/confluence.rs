#[derive(Clone, Copy, Debug)]
pub struct GetAttachmentByIDReplyLinks;

impl GetAttachmentByIDReplyLinks {
    pub const BASE: crate::Field<Self, crate::Text> = crate::Field::new("base");
    pub const DOWNLOAD: crate::Field<Self, crate::Text> = crate::Field::new("download");
    pub const WEBUI: crate::Field<Self, crate::Text> = crate::Field::new("webui");
}
