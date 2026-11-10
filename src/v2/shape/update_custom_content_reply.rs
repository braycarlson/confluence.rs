#[derive(Clone, Copy, Debug)]
pub struct UpdateCustomContentReply;

impl UpdateCustomContentReply {
    pub const AUTHOR_ID: crate::Field<Self, crate::Text> = crate::Field::new("authorId");
    pub const BLOG_POST_ID: crate::Field<Self, crate::Text> = crate::Field::new("blogPostId");

    pub const BODY: crate::Field<Self, crate::Object<super::CustomContentBodySingle>> =
        crate::Field::new("body");

    pub const CREATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("createdAt");

    pub const CUSTOM_CONTENT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("customContentId");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LABELS: crate::Field<Self, crate::Object<super::UpdateCustomContentReplyLabels>> =
        crate::Field::new("labels");

    pub const LINKS: crate::Field<Self, crate::Object<super::UpdateCustomContentReplyLinks>> =
        crate::Field::new("_links");

    pub const OPERATIONS: crate::Field<
        Self,
        crate::Object<super::UpdateCustomContentReplyOperations>,
    > = crate::Field::new("operations");

    pub const PAGE_ID: crate::Field<Self, crate::Text> = crate::Field::new("pageId");

    pub const PROPERTIES: crate::Field<
        Self,
        crate::Object<super::UpdateCustomContentReplyProperties>,
    > = crate::Field::new("properties");

    pub const SPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("spaceId");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const VERSION: crate::Field<Self, crate::Object<super::Version>> =
        crate::Field::new("version");

    pub const VERSIONS: crate::Field<Self, crate::Object<super::UpdateCustomContentReplyVersions>> =
        crate::Field::new("versions");
}
