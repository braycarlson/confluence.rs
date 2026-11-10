#[derive(Clone, Copy, Debug)]
pub struct GetBlogPostLikeCountReply;

impl GetBlogPostLikeCountReply {
    pub const COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("count");
}
