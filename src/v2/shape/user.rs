#[derive(Clone, Copy, Debug)]
pub struct User;

impl User {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const ACCOUNT_STATUS: crate::Field<Self, crate::Text> = crate::Field::new("accountStatus");
    pub const ACCOUNT_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("accountType");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const EMAIL: crate::Field<Self, crate::Text> = crate::Field::new("email");

    pub const IS_EXTERNAL_COLLABORATOR: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isExternalCollaborator");

    pub const PERSONAL_SPACE_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("personalSpaceId");

    pub const PROFILE_PICTURE: crate::Field<Self, crate::Object<super::Icon>> =
        crate::Field::new("profilePicture");

    pub const PUBLIC_NAME: crate::Field<Self, crate::Text> = crate::Field::new("publicName");
    pub const TIME_ZONE: crate::Field<Self, crate::Text> = crate::Field::new("timeZone");
}
