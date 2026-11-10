#[derive(Clone, Copy, Debug)]
pub struct GetPageByIDReplyCollaborators;

impl GetPageByIDReplyCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
