#[derive(Clone, Copy, Debug)]
pub struct PageSingleCollaborators;

impl PageSingleCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
