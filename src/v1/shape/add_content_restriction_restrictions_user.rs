#[derive(Clone, Copy, Debug)]
pub struct AddContentRestrictionRestrictionsUser;

impl AddContentRestrictionRestrictionsUser {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const USERNAME: crate::Field<Self, crate::Text> = crate::Field::new("username");
    pub const USER_KEY: crate::Field<Self, crate::Text> = crate::Field::new("userKey");
}
