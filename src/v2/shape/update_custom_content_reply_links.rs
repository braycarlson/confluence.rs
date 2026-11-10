#[derive(Clone, Copy, Debug)]
pub struct UpdateCustomContentReplyLinks;

impl UpdateCustomContentReplyLinks {
    pub const BASE: crate::Field<Self, crate::Text> = crate::Field::new("base");
    pub const WEBUI: crate::Field<Self, crate::Text> = crate::Field::new("webui");
}
