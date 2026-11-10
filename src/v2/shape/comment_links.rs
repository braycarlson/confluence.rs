#[derive(Clone, Copy, Debug)]
pub struct CommentLinks;

impl CommentLinks {
    pub const WEBUI: crate::Field<Self, crate::Text> = crate::Field::new("webui");
}
