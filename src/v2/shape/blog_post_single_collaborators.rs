#[derive(Clone, Copy, Debug)]
pub struct BlogPostSingleCollaborators;

impl BlogPostSingleCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
