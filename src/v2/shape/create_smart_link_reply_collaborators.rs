#[derive(Clone, Copy, Debug)]
pub struct CreateSmartLinkReplyCollaborators;

impl CreateSmartLinkReplyCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
