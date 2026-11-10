#[derive(Clone, Copy, Debug)]
pub struct GetFolderByIDReplyCollaborators;

impl GetFolderByIDReplyCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
