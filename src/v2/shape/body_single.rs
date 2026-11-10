#[derive(Clone, Copy, Debug)]
pub struct BodySingle;

impl BodySingle {
    pub const ATLAS_DOC_FORMAT: crate::Field<Self, crate::Object<super::BodyType>> =
        crate::Field::new("atlas_doc_format");

    pub const STORAGE: crate::Field<Self, crate::Object<super::BodyType>> =
        crate::Field::new("storage");

    pub const VIEW: crate::Field<Self, crate::Object<super::BodyType>> = crate::Field::new("view");
}
