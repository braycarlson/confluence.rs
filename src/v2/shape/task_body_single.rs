#[derive(Clone, Copy, Debug)]
pub struct TaskBodySingle;

impl TaskBodySingle {
    pub const ATLAS_DOC_FORMAT: crate::Field<Self, crate::Object<super::BodyType>> =
        crate::Field::new("atlas_doc_format");

    pub const STORAGE: crate::Field<Self, crate::Object<super::BodyType>> =
        crate::Field::new("storage");
}
