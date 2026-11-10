#[derive(Clone, Copy, Debug)]
pub struct AdminKeyResponse;

impl AdminKeyResponse {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");

    pub const EXPIRATION_TIME: crate::Field<Self, crate::Text> =
        crate::Field::new("expirationTime");
}
