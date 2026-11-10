#[derive(Clone, Copy, Debug)]
pub struct InlineCommentModel;

impl InlineCommentModel {
    pub const BLOG_POST_ID: crate::Field<Self, crate::Text> = crate::Field::new("blogPostId");

    pub const BODY: crate::Field<Self, crate::Object<super::BodySingle>> =
        crate::Field::new("body");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LIKES: crate::Field<Self, crate::Object<super::InlineCommentModelLikes>> =
        crate::Field::new("likes");

    pub const LINKS: crate::Field<Self, crate::Object<super::CommentLinks>> =
        crate::Field::new("_links");

    pub const OPERATIONS: crate::Field<Self, crate::Object<super::InlineCommentModelOperations>> =
        crate::Field::new("operations");

    pub const PAGE_ID: crate::Field<Self, crate::Text> = crate::Field::new("pageId");

    pub const PARENT_COMMENT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("parentCommentId");

    pub const PROPERTIES: crate::Field<Self, crate::Object<super::InlineCommentModelProperties>> =
        crate::Field::new("properties");

    pub const RESOLUTION_MODIFIED_AT_LAST: crate::Field<Self, crate::Text> =
        crate::Field::new("resolutionLastModifiedAt");

    pub const RESOLUTION_MODIFIER_ID_LAST: crate::Field<Self, crate::Text> =
        crate::Field::new("resolutionLastModifierId");

    pub const RESOLUTION_STATUS: crate::Field<Self, crate::Text> =
        crate::Field::new("resolutionStatus");

    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");

    pub const VERSION: crate::Field<Self, crate::Object<super::Version>> =
        crate::Field::new("version");

    pub const VERSIONS: crate::Field<Self, crate::Object<super::InlineCommentModelVersions>> =
        crate::Field::new("versions");
}
