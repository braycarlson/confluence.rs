#[derive(Clone, Copy, Debug)]
pub struct DatabaseUserRef;

impl DatabaseUserRef {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
