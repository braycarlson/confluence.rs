#[derive(Clone, Copy, Debug)]
pub struct ContentChildren;

impl ContentChildren {
    pub const ATTACHMENT: crate::Field<Self, crate::Object<super::ContentArray>> =
        crate::Field::new("attachment");

    pub const COMMENT: crate::Field<Self, crate::Object<super::ContentArray>> =
        crate::Field::new("comment");

    pub const DATABASE: crate::Field<Self, crate::Object<super::ContentArray>> =
        crate::Field::new("database");

    pub const EMBED: crate::Field<Self, crate::Object<super::ContentArray>> =
        crate::Field::new("embed");

    pub const EXPANDABLE: crate::Field<Self, crate::Object<super::ContentChildrenExpandable>> =
        crate::Field::new("_expandable");

    pub const FOLDER: crate::Field<Self, crate::Object<super::ContentArray>> =
        crate::Field::new("folder");

    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");

    pub const PAGE: crate::Field<Self, crate::Object<super::ContentArray>> =
        crate::Field::new("page");

    pub const WHITEBOARD: crate::Field<Self, crate::Object<super::ContentArray>> =
        crate::Field::new("whiteboard");
}
