#[derive(Clone, Copy, Debug)]
pub struct GetInlineLikeCountReply;

impl GetInlineLikeCountReply {
    pub const COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("count");
}
