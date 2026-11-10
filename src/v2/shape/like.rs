#[derive(Clone, Copy, Debug)]
pub struct Like;

impl Like {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
}
