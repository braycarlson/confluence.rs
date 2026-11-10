#[derive(Clone, Copy, Debug)]
pub struct UpdatePageReplyCollaborators;

impl UpdatePageReplyCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
