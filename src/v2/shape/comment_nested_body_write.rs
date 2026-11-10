#[derive(Clone, Copy, Debug)]
pub struct CommentNestedBodyWrite;

impl CommentNestedBodyWrite {
    pub const ATLAS_DOC_FORMAT: crate::Field<Self, crate::Object<super::CommentBodyWrite>> =
        crate::Field::new("atlas_doc_format");

    pub const STORAGE: crate::Field<Self, crate::Object<super::CommentBodyWrite>> =
        crate::Field::new("storage");

    pub const WIKI: crate::Field<Self, crate::Object<super::CommentBodyWrite>> =
        crate::Field::new("wiki");
}
