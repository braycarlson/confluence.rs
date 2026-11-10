#[derive(Clone, Copy, Debug)]
pub struct ContentRestrictionUserInput;

impl ContentRestrictionUserInput {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
