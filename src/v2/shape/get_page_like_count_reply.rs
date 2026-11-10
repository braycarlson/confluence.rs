#[derive(Clone, Copy, Debug)]
pub struct GetPageLikeCountReply;

impl GetPageLikeCountReply {
    pub const COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("count");
}
