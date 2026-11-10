#[derive(Clone, Copy, Debug)]
pub struct SpaceSingle;

impl SpaceSingle {
    pub const ACTIVE_ALIAS_CURRENT: crate::Field<Self, crate::Text> =
        crate::Field::new("currentActiveAlias");

    pub const AUTHOR_ID: crate::Field<Self, crate::Text> = crate::Field::new("authorId");
    pub const CREATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("createdAt");

    pub const DESCRIPTION: crate::Field<Self, crate::Object<super::SpaceDescription>> =
        crate::Field::new("description");

    pub const HOMEPAGE_ID: crate::Field<Self, crate::Text> = crate::Field::new("homepageId");
    pub const ICON: crate::Field<Self, crate::Object<super::SpaceIcon>> = crate::Field::new("icon");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");

    pub const LABELS: crate::Field<Self, crate::Object<super::SpaceSingleLabels>> =
        crate::Field::new("labels");

    pub const LINKS: crate::Field<Self, crate::Object<super::SpaceLinks>> =
        crate::Field::new("_links");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const OPERATIONS: crate::Field<Self, crate::Object<super::SpaceSingleOperations>> =
        crate::Field::new("operations");

    pub const PERMISSIONS: crate::Field<Self, crate::Object<super::SpaceSinglePermissions>> =
        crate::Field::new("permissions");

    pub const PROPERTIES: crate::Field<Self, crate::Object<super::SpaceSingleProperties>> =
        crate::Field::new("properties");

    pub const SPACE_OWNER_ID: crate::Field<Self, crate::Text> = crate::Field::new("spaceOwnerId");

    pub const SPACE_ROLE_ASSIGNMENTS: crate::Field<
        Self,
        crate::Object<super::SpaceSingleSpaceRoleAssignments>,
    > = crate::Field::new("spaceRoleAssignments");

    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
