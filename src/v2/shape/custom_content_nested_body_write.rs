#[derive(Clone, Copy, Debug)]
pub struct CustomContentNestedBodyWrite;

impl CustomContentNestedBodyWrite {
    pub const ATLAS_DOC_FORMAT: crate::Field<Self, crate::Object<super::CustomContentBodyWrite>> =
        crate::Field::new("atlas_doc_format");

    pub const RAW: crate::Field<Self, crate::Object<super::CustomContentBodyWrite>> =
        crate::Field::new("raw");

    pub const STORAGE: crate::Field<Self, crate::Object<super::CustomContentBodyWrite>> =
        crate::Field::new("storage");
}
