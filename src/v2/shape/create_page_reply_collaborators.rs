#[derive(Clone, Copy, Debug)]
pub struct CreatePageReplyCollaborators;

impl CreatePageReplyCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
