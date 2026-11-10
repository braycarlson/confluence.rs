#[derive(Clone, Copy, Debug)]
pub struct GetFooterLikeCountReply;

impl GetFooterLikeCountReply {
    pub const COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("count");
}
