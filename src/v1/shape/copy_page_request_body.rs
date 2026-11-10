#[derive(Clone, Copy, Debug)]
pub struct CopyPageRequestBody;

impl CopyPageRequestBody {
    pub const EDITOR2: crate::Field<Self, crate::Object<super::ContentBodyCreate>> =
        crate::Field::new("editor2");

    pub const STORAGE: crate::Field<Self, crate::Object<super::ContentBodyCreate>> =
        crate::Field::new("storage");
}
