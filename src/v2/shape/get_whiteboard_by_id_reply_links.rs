#[derive(Clone, Copy, Debug)]
pub struct GetWhiteboardByIDReplyLinks;

impl GetWhiteboardByIDReplyLinks {
    pub const BASE: crate::Field<Self, crate::Text> = crate::Field::new("base");
    pub const EDITUI: crate::Field<Self, crate::Text> = crate::Field::new("editui");
    pub const EDITUIV2: crate::Field<Self, crate::Text> = crate::Field::new("edituiv2");
    pub const TINYUI: crate::Field<Self, crate::Text> = crate::Field::new("tinyui");
    pub const WEBUI: crate::Field<Self, crate::Text> = crate::Field::new("webui");
}
