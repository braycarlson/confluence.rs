#[derive(Clone, Copy, Debug)]
pub struct GetDatabaseByIDReply;

impl GetDatabaseByIDReply {
    pub const AUTHOR_ID: crate::Field<Self, crate::Text> = crate::Field::new("authorId");

    pub const COLLABORATORS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetDatabaseByIDReplyCollaborators>>,
    > = crate::Field::new("collaborators");

    pub const CREATED_AT: crate::Field<Self, crate::Integer> = crate::Field::new("createdAt");

    pub const DIRECT_CHILDREN: crate::Field<
        Self,
        crate::Object<super::GetDatabaseByIDReplyDirectChildren>,
    > = crate::Field::new("directChildren");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LINKS: crate::Field<Self, crate::Object<super::GetDatabaseByIDReplyLinks>> =
        crate::Field::new("_links");

    pub const OPERATIONS: crate::Field<Self, crate::Object<super::GetDatabaseByIDReplyOperations>> =
        crate::Field::new("operations");

    pub const OWNER_ID: crate::Field<Self, crate::Text> = crate::Field::new("ownerId");
    pub const PARENT_ID: crate::Field<Self, crate::Text> = crate::Field::new("parentId");
    pub const PARENT_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("parentType");
    pub const POSITION: crate::Field<Self, crate::Integer> = crate::Field::new("position");

    pub const PROPERTIES: crate::Field<Self, crate::Object<super::GetDatabaseByIDReplyProperties>> =
        crate::Field::new("properties");

    pub const SPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("spaceId");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const VERSION: crate::Field<Self, crate::Object<super::Version>> =
        crate::Field::new("version");
}
