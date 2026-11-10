#[derive(Clone, Copy, Debug)]
pub struct UpdateBlogPostReplyCollaborators;

impl UpdateBlogPostReplyCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
