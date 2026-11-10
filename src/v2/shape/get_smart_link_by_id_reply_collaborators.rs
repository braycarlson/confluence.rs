#[derive(Clone, Copy, Debug)]
pub struct GetSmartLinkByIDReplyCollaborators;

impl GetSmartLinkByIDReplyCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
