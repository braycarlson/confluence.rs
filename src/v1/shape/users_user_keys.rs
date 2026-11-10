#[derive(Clone, Copy, Debug)]
pub struct UsersUserKeys;

impl UsersUserKeys {
    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");

    pub const USERS: crate::Field<Self, crate::List<crate::Object<super::User>>> =
        crate::Field::new("users");

    pub const USER_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("userKeys");
}
