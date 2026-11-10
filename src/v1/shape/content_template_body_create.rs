#[derive(Clone, Copy, Debug)]
pub struct ContentTemplateBodyCreate;

impl ContentTemplateBodyCreate {
    pub const ANONYMOUS_EXPORT_VIEW: crate::Field<Self, crate::Object<super::ContentBodyCreate>> =
        crate::Field::new("anonymous_export_view");

    pub const ATLAS_DOC_FORMAT: crate::Field<Self, crate::Object<super::ContentBodyCreate>> =
        crate::Field::new("atlas_doc_format");

    pub const EDITOR: crate::Field<Self, crate::Object<super::ContentBodyCreate>> =
        crate::Field::new("editor");

    pub const EDITOR2: crate::Field<Self, crate::Object<super::ContentBodyCreate>> =
        crate::Field::new("editor2");

    pub const EXPORT_VIEW: crate::Field<Self, crate::Object<super::ContentBodyCreate>> =
        crate::Field::new("export_view");

    pub const STORAGE: crate::Field<Self, crate::Object<super::ContentBodyCreate>> =
        crate::Field::new("storage");

    pub const STYLED_VIEW: crate::Field<Self, crate::Object<super::ContentBodyCreate>> =
        crate::Field::new("styled_view");

    pub const VIEW: crate::Field<Self, crate::Object<super::ContentBodyCreate>> =
        crate::Field::new("view");

    pub const WIKI: crate::Field<Self, crate::Object<super::ContentBodyCreate>> =
        crate::Field::new("wiki");
}
