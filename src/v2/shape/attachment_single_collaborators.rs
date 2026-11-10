#[derive(Clone, Copy, Debug)]
pub struct AttachmentSingleCollaborators;

impl AttachmentSingleCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
