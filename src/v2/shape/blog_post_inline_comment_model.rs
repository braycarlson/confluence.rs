#[derive(Clone, Copy, Debug)]
pub struct BlogPostInlineCommentModel;

impl BlogPostInlineCommentModel {
    pub const BLOG_POST_ID: crate::Field<Self, crate::Text> = crate::Field::new("blogPostId");
    pub const BODY: crate::Field<Self, crate::Object<super::BodyBulk>> = crate::Field::new("body");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LINKS: crate::Field<Self, crate::Object<super::CommentLinks>> =
        crate::Field::new("_links");

    pub const PROPERTIES: crate::Field<Self, crate::Object<super::InlineCommentProperties>> =
        crate::Field::new("properties");

    pub const RESOLUTION_STATUS: crate::Field<Self, crate::Text> =
        crate::Field::new("resolutionStatus");

    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");

    pub const VERSION: crate::Field<Self, crate::Object<super::Version>> =
        crate::Field::new("version");
}
