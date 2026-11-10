#[derive(Clone, Copy, Debug)]
pub struct SmartLinkSingleCollaborators;

impl SmartLinkSingleCollaborators {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
