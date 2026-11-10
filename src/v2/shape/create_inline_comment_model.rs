#[derive(Clone, Copy, Debug)]
pub struct CreateInlineCommentModel;

impl CreateInlineCommentModel {
    pub const BLOG_POST_ID: crate::Field<Self, crate::Text> = crate::Field::new("blogPostId");
    pub const BODY: crate::Field<Self, crate::Any> = crate::Field::new("body");

    pub const INLINE_COMMENT_PROPERTIES: crate::Field<
        Self,
        crate::Object<super::CreateInlineCommentModelInlineCommentProperties>,
    > = crate::Field::new("inlineCommentProperties");

    pub const PAGE_ID: crate::Field<Self, crate::Text> = crate::Field::new("pageId");

    pub const PARENT_COMMENT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("parentCommentId");
}
