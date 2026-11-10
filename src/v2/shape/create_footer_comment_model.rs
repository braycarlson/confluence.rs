#[derive(Clone, Copy, Debug)]
pub struct CreateFooterCommentModel;

impl CreateFooterCommentModel {
    pub const ATTACHMENT_ID: crate::Field<Self, crate::Text> = crate::Field::new("attachmentId");
    pub const BLOG_POST_ID: crate::Field<Self, crate::Text> = crate::Field::new("blogPostId");
    pub const BODY: crate::Field<Self, crate::Any> = crate::Field::new("body");

    pub const CUSTOM_CONTENT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("customContentId");

    pub const PAGE_ID: crate::Field<Self, crate::Text> = crate::Field::new("pageId");

    pub const PARENT_COMMENT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("parentCommentId");
}
