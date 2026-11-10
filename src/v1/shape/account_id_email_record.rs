#[derive(Clone, Copy, Debug)]
pub struct AccountIDEmailRecord;

impl AccountIDEmailRecord {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const EMAIL: crate::Field<Self, crate::Text> = crate::Field::new("email");
}
