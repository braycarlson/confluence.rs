#[derive(Clone, Copy, Debug)]
pub struct GetBlogPostByIDReplyCollaborators;

impl GetBlogPostByIDReplyCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
