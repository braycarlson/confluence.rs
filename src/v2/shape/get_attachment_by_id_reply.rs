#[derive(Clone, Copy, Debug)]
pub struct GetAttachmentByIDReply;

impl GetAttachmentByIDReply {
    pub const BLOG_POST_ID: crate::Field<Self, crate::Text> = crate::Field::new("blogPostId");

    pub const COLLABORATORS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetAttachmentByIDReplyCollaborators>>,
    > = crate::Field::new("collaborators");

    pub const COMMENT: crate::Field<Self, crate::Text> = crate::Field::new("comment");
    pub const CREATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("createdAt");

    pub const CUSTOM_CONTENT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("customContentId");

    pub const DOWNLOAD_LINK: crate::Field<Self, crate::Text> = crate::Field::new("downloadLink");
    pub const FILE_ID: crate::Field<Self, crate::Text> = crate::Field::new("fileId");
    pub const FILE_SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("fileSize");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LABELS: crate::Field<Self, crate::Object<super::GetAttachmentByIDReplyLabels>> =
        crate::Field::new("labels");

    pub const LINKS: crate::Field<Self, crate::Object<super::GetAttachmentByIDReplyLinks>> =
        crate::Field::new("_links");

    pub const MEDIA_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("mediaType");

    pub const MEDIA_TYPE_DESCRIPTION: crate::Field<Self, crate::Text> =
        crate::Field::new("mediaTypeDescription");

    pub const OPERATIONS: crate::Field<
        Self,
        crate::Object<super::GetAttachmentByIDReplyOperations>,
    > = crate::Field::new("operations");

    pub const PAGE_ID: crate::Field<Self, crate::Text> = crate::Field::new("pageId");

    pub const PROPERTIES: crate::Field<
        Self,
        crate::Object<super::GetAttachmentByIDReplyProperties>,
    > = crate::Field::new("properties");

    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");

    pub const VERSION: crate::Field<Self, crate::Object<super::Version>> =
        crate::Field::new("version");

    pub const VERSIONS: crate::Field<Self, crate::Object<super::GetAttachmentByIDReplyVersions>> =
        crate::Field::new("versions");

    pub const WEBUI_LINK: crate::Field<Self, crate::Text> = crate::Field::new("webuiLink");
}
