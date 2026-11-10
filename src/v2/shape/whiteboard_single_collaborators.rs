#[derive(Clone, Copy, Debug)]
pub struct WhiteboardSingleCollaborators;

impl WhiteboardSingleCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
