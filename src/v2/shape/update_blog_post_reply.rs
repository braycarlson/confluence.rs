#[derive(Clone, Copy, Debug)]
pub struct UpdateBlogPostReply;

impl UpdateBlogPostReply {
    pub const AUTHOR_ID: crate::Field<Self, crate::Text> = crate::Field::new("authorId");

    pub const BODY: crate::Field<Self, crate::Object<super::BodySingle>> =
        crate::Field::new("body");

    pub const COLLABORATORS: crate::Field<
        Self,
        crate::List<crate::Object<super::UpdateBlogPostReplyCollaborators>>,
    > = crate::Field::new("collaborators");

    pub const CREATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("createdAt");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const IS_FAVORITED_BY_USER_CURRENT: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isFavoritedByCurrentUser");

    pub const LABELS: crate::Field<Self, crate::Object<super::UpdateBlogPostReplyLabels>> =
        crate::Field::new("labels");

    pub const LIKES: crate::Field<Self, crate::Object<super::UpdateBlogPostReplyLikes>> =
        crate::Field::new("likes");

    pub const LINKS: crate::Field<Self, crate::Object<super::UpdateBlogPostReplyLinks>> =
        crate::Field::new("_links");

    pub const OPERATIONS: crate::Field<Self, crate::Object<super::UpdateBlogPostReplyOperations>> =
        crate::Field::new("operations");

    pub const PROPERTIES: crate::Field<Self, crate::Object<super::UpdateBlogPostReplyProperties>> =
        crate::Field::new("properties");

    pub const SPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("spaceId");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");

    pub const VERSION: crate::Field<Self, crate::Object<super::Version>> =
        crate::Field::new("version");

    pub const VERSIONS: crate::Field<Self, crate::Object<super::UpdateBlogPostReplyVersions>> =
        crate::Field::new("versions");

    pub const WEB_RESOURCES: crate::Field<Self, crate::Any> = crate::Field::new("webResources");
}
