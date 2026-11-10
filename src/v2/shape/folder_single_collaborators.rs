#[derive(Clone, Copy, Debug)]
pub struct FolderSingleCollaborators;

impl FolderSingleCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
