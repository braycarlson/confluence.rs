#[derive(Clone, Copy, Debug)]
pub struct GetAttachmentByIDReplyCollaborators;

impl GetAttachmentByIDReplyCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
