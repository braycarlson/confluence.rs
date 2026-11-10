#[derive(Clone, Copy, Debug)]
pub struct GetDatabaseByIDReplyCollaborators;

impl GetDatabaseByIDReplyCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
