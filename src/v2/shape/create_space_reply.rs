#[derive(Clone, Copy, Debug)]
pub struct CreateSpaceReply;

impl CreateSpaceReply {
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

    pub const LINKS: crate::Field<Self, crate::Object<super::CreateSpaceReplyLinks>> =
        crate::Field::new("_links");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SPACE_OWNER_ID: crate::Field<Self, crate::Text> = crate::Field::new("spaceOwnerId");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
