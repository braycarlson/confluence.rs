#[derive(Clone, Copy, Debug)]
pub struct PageSingle;

impl PageSingle {
    pub const AUTHOR_ID: crate::Field<Self, crate::Text> = crate::Field::new("authorId");

    pub const BODY: crate::Field<Self, crate::Object<super::BodySingle>> =
        crate::Field::new("body");

    pub const COLLABORATORS: crate::Field<
        Self,
        crate::List<crate::Object<super::PageSingleCollaborators>>,
    > = crate::Field::new("collaborators");

    pub const CREATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("createdAt");

    pub const DIRECT_CHILDREN: crate::Field<Self, crate::Object<super::PageSingleDirectChildren>> =
        crate::Field::new("directChildren");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const IS_FAVORITED_BY_USER_CURRENT: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isFavoritedByCurrentUser");

    pub const LABELS: crate::Field<Self, crate::Object<super::PageSingleLabels>> =
        crate::Field::new("labels");

    pub const LIKES: crate::Field<Self, crate::Object<super::PageSingleLikes>> =
        crate::Field::new("likes");

    pub const LINKS: crate::Field<Self, crate::Object<super::AbstractPageLinks>> =
        crate::Field::new("_links");

    pub const OPERATIONS: crate::Field<Self, crate::Object<super::PageSingleOperations>> =
        crate::Field::new("operations");

    pub const OWNER_ID: crate::Field<Self, crate::Text> = crate::Field::new("ownerId");
    pub const OWNER_ID_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastOwnerId");
    pub const PARENT_ID: crate::Field<Self, crate::Text> = crate::Field::new("parentId");
    pub const PARENT_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("parentType");
    pub const POSITION: crate::Field<Self, crate::Integer> = crate::Field::new("position");

    pub const PROPERTIES: crate::Field<Self, crate::Object<super::PageSingleProperties>> =
        crate::Field::new("properties");

    pub const SPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("spaceId");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");

    pub const VERSION: crate::Field<Self, crate::Object<super::Version>> =
        crate::Field::new("version");

    pub const VERSIONS: crate::Field<Self, crate::Object<super::PageSingleVersions>> =
        crate::Field::new("versions");

    pub const WEB_RESOURCES: crate::Field<Self, crate::Any> = crate::Field::new("webResources");
}
