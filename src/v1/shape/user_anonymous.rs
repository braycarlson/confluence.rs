#[derive(Clone, Copy, Debug)]
pub struct UserAnonymous;

impl UserAnonymous {
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");

    pub const EXPANDABLE: crate::Field<Self, crate::Object<super::UserAnonymousExpandable>> =
        crate::Field::new("_expandable");

    pub const IS_EXTERNAL_COLLABORATOR: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isExternalCollaborator");

    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");

    pub const OPERATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::OperationCheckResult>>,
    > = crate::Field::new("operations");

    pub const PROFILE_PICTURE: crate::Field<Self, crate::Object<super::Icon>> =
        crate::Field::new("profilePicture");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
