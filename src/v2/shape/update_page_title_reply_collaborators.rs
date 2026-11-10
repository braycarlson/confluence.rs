#[derive(Clone, Copy, Debug)]
pub struct UpdatePageTitleReplyCollaborators;

impl UpdatePageTitleReplyCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
