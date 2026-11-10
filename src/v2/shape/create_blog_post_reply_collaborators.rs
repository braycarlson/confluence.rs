#[derive(Clone, Copy, Debug)]
pub struct CreateBlogPostReplyCollaborators;

impl CreateBlogPostReplyCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
