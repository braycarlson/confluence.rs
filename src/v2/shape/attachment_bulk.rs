#[derive(Clone, Copy, Debug)]
pub struct AttachmentBulk;

impl AttachmentBulk {
    pub const BLOG_POST_ID: crate::Field<Self, crate::Text> = crate::Field::new("blogPostId");
    pub const COMMENT: crate::Field<Self, crate::Text> = crate::Field::new("comment");
    pub const CREATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("createdAt");

    pub const CUSTOM_CONTENT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("customContentId");

    pub const DOWNLOAD_LINK: crate::Field<Self, crate::Text> = crate::Field::new("downloadLink");
    pub const FILE_ID: crate::Field<Self, crate::Text> = crate::Field::new("fileId");
    pub const FILE_SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("fileSize");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LINKS: crate::Field<Self, crate::Object<super::AttachmentLinks>> =
        crate::Field::new("_links");

    pub const MEDIA_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("mediaType");

    pub const MEDIA_TYPE_DESCRIPTION: crate::Field<Self, crate::Text> =
        crate::Field::new("mediaTypeDescription");

    pub const PAGE_ID: crate::Field<Self, crate::Text> = crate::Field::new("pageId");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");

    pub const VERSION: crate::Field<Self, crate::Object<super::Version>> =
        crate::Field::new("version");

    pub const WEBUI_LINK: crate::Field<Self, crate::Text> = crate::Field::new("webuiLink");
}
