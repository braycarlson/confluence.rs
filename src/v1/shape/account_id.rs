#[derive(Clone, Copy, Debug)]
pub struct AccountID;

impl AccountID {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
