#[derive(Clone, Copy, Debug)]
pub struct ContentChildType;

impl ContentChildType {
    pub const ATTACHMENT: crate::Field<Self, crate::Object<super::ContentChildTypeAttachment>> =
        crate::Field::new("attachment");

    pub const COMMENT: crate::Field<Self, crate::Object<super::ContentChildTypeComment>> =
        crate::Field::new("comment");

    pub const EXPANDABLE: crate::Field<Self, crate::Object<super::ContentChildTypeExpandable>> =
        crate::Field::new("_expandable");

    pub const PAGE: crate::Field<Self, crate::Object<super::ContentChildTypePage>> =
        crate::Field::new("page");
}
