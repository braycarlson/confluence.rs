#[derive(Clone, Copy, Debug)]
pub struct DatabaseSingleCollaborators;

impl DatabaseSingleCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
