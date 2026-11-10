#[derive(Clone, Copy, Debug)]
pub struct MovePageReply;

impl MovePageReply {
    pub const PAGE_ID: crate::Field<Self, crate::Text> = crate::Field::new("pageId");
}
