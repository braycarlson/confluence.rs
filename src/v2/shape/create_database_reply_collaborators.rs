#[derive(Clone, Copy, Debug)]
pub struct CreateDatabaseReplyCollaborators;

impl CreateDatabaseReplyCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
