#[derive(Clone, Copy, Debug)]
pub struct GetBlogPostByIDReply;

impl GetBlogPostByIDReply {
    pub const AUTHOR_ID: crate::Field<Self, crate::Text> = crate::Field::new("authorId");

    pub const BODY: crate::Field<Self, crate::Object<super::BodySingle>> =
        crate::Field::new("body");

    pub const COLLABORATORS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetBlogPostByIDReplyCollaborators>>,
    > = crate::Field::new("collaborators");

    pub const CREATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("createdAt");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const IS_FAVORITED_BY_USER_CURRENT: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isFavoritedByCurrentUser");

    pub const LABELS: crate::Field<Self, crate::Object<super::GetBlogPostByIDReplyLabels>> =
        crate::Field::new("labels");

    pub const LIKES: crate::Field<Self, crate::Object<super::GetBlogPostByIDReplyLikes>> =
        crate::Field::new("likes");

    pub const LINKS: crate::Field<Self, crate::Object<super::GetBlogPostByIDReplyLinks>> =
        crate::Field::new("_links");

    pub const OPERATIONS: crate::Field<Self, crate::Object<super::GetBlogPostByIDReplyOperations>> =
        crate::Field::new("operations");

    pub const PROPERTIES: crate::Field<Self, crate::Object<super::GetBlogPostByIDReplyProperties>> =
        crate::Field::new("properties");

    pub const SPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("spaceId");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");

    pub const VERSION: crate::Field<Self, crate::Object<super::Version>> =
        crate::Field::new("version");

    pub const VERSIONS: crate::Field<Self, crate::Object<super::GetBlogPostByIDReplyVersions>> =
        crate::Field::new("versions");

    pub const WEB_RESOURCES: crate::Field<Self, crate::Any> = crate::Field::new("webResources");
}
