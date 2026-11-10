#[derive(Clone, Copy, Debug)]
pub struct BlogPostNestedBodyWrite;

impl BlogPostNestedBodyWrite {
    pub const ATLAS_DOC_FORMAT: crate::Field<Self, crate::Object<super::BlogPostBodyWrite>> =
        crate::Field::new("atlas_doc_format");

    pub const STORAGE: crate::Field<Self, crate::Object<super::BlogPostBodyWrite>> =
        crate::Field::new("storage");

    pub const WIKI: crate::Field<Self, crate::Object<super::BlogPostBodyWrite>> =
        crate::Field::new("wiki");
}
