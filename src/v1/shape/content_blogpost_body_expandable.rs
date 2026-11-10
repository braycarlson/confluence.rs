#[derive(Clone, Copy, Debug)]
pub struct ContentBlogpostBodyExpandable;

impl ContentBlogpostBodyExpandable {
    pub const ANONYMOUS_EXPORT_VIEW: crate::Field<Self, crate::Text> =
        crate::Field::new("anonymous_export_view");

    pub const ATLAS_DOC_FORMAT: crate::Field<Self, crate::Text> =
        crate::Field::new("atlas_doc_format");

    pub const DYNAMIC: crate::Field<Self, crate::Text> = crate::Field::new("dynamic");
    pub const EDITOR: crate::Field<Self, crate::Text> = crate::Field::new("editor");
    pub const EDITOR2: crate::Field<Self, crate::Text> = crate::Field::new("editor2");
    pub const EXPORT_VIEW: crate::Field<Self, crate::Text> = crate::Field::new("export_view");
    pub const MARKDOWN: crate::Field<Self, crate::Text> = crate::Field::new("markdown");
    pub const RAW: crate::Field<Self, crate::Text> = crate::Field::new("raw");
    pub const STORAGE: crate::Field<Self, crate::Text> = crate::Field::new("storage");
    pub const STYLED_VIEW: crate::Field<Self, crate::Text> = crate::Field::new("styled_view");
    pub const VIEW: crate::Field<Self, crate::Text> = crate::Field::new("view");
    pub const WIKI: crate::Field<Self, crate::Text> = crate::Field::new("wiki");
}
