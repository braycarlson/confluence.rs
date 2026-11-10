#[derive(Clone, Copy, Debug)]
pub struct BulkUserLookup;

impl BulkUserLookup {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const ACCOUNT_STATUS: crate::Field<Self, crate::Text> = crate::Field::new("accountStatus");
    pub const ACCOUNT_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("accountType");

    pub const DETAILS: crate::Field<Self, crate::Object<super::UserDetails>> =
        crate::Field::new("details");

    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const EMAIL: crate::Field<Self, crate::Text> = crate::Field::new("email");

    pub const EXPANDABLE: crate::Field<Self, crate::Object<super::BulkUserLookupExpandable>> =
        crate::Field::new("_expandable");

    pub const IS_EXTERNAL_COLLABORATOR: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isExternalCollaborator");

    pub const IS_GUEST: crate::Field<Self, crate::Boolean> = crate::Field::new("isGuest");
    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");
    pub const LOCALE: crate::Field<Self, crate::Text> = crate::Field::new("locale");

    pub const OPERATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::OperationCheckResult>>,
    > = crate::Field::new("operations");

    pub const PERSONAL_SPACE: crate::Field<Self, crate::Object<super::Space>> =
        crate::Field::new("personalSpace");

    pub const PROFILE_PICTURE: crate::Field<Self, crate::Object<super::Icon>> =
        crate::Field::new("profilePicture");

    pub const PUBLIC_NAME: crate::Field<Self, crate::Text> = crate::Field::new("publicName");
    pub const TIME_ZONE: crate::Field<Self, crate::Text> = crate::Field::new("timeZone");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const USERNAME: crate::Field<Self, crate::Text> = crate::Field::new("username");
    pub const USER_KEY: crate::Field<Self, crate::Text> = crate::Field::new("userKey");
}
