#[derive(Clone, Copy, Debug)]
pub struct ContentTemplateBody;

impl ContentTemplateBody {
    pub const ANONYMOUS_EXPORT_VIEW: crate::Field<Self, crate::Object<super::ContentBody>> =
        crate::Field::new("anonymous_export_view");

    pub const ATLAS_DOC_FORMAT: crate::Field<Self, crate::Object<super::ContentBody>> =
        crate::Field::new("atlas_doc_format");

    pub const EDITOR: crate::Field<Self, crate::Object<super::ContentBody>> =
        crate::Field::new("editor");

    pub const EDITOR2: crate::Field<Self, crate::Object<super::ContentBody>> =
        crate::Field::new("editor2");

    pub const EXPORT_VIEW: crate::Field<Self, crate::Object<super::ContentBody>> =
        crate::Field::new("export_view");

    pub const STORAGE: crate::Field<Self, crate::Object<super::ContentBody>> =
        crate::Field::new("storage");

    pub const STYLED_VIEW: crate::Field<Self, crate::Object<super::ContentBody>> =
        crate::Field::new("styled_view");

    pub const VIEW: crate::Field<Self, crate::Object<super::ContentBody>> =
        crate::Field::new("view");

    pub const WIKI: crate::Field<Self, crate::Object<super::ContentBody>> =
        crate::Field::new("wiki");
}
