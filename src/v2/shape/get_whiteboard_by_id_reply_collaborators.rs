#[derive(Clone, Copy, Debug)]
pub struct GetWhiteboardByIDReplyCollaborators;

impl GetWhiteboardByIDReplyCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
