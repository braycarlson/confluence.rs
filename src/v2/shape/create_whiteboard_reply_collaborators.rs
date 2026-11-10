#[derive(Clone, Copy, Debug)]
pub struct CreateWhiteboardReplyCollaborators;

impl CreateWhiteboardReplyCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
