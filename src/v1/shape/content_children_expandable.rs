#[derive(Clone, Copy, Debug)]
pub struct ContentChildrenExpandable;

impl ContentChildrenExpandable {
    pub const ATTACHMENT: crate::Field<Self, crate::Text> = crate::Field::new("attachment");
    pub const COMMENT: crate::Field<Self, crate::Text> = crate::Field::new("comment");
    pub const DATABASE: crate::Field<Self, crate::Text> = crate::Field::new("database");
    pub const EMBED: crate::Field<Self, crate::Text> = crate::Field::new("embed");
    pub const FOLDER: crate::Field<Self, crate::Text> = crate::Field::new("folder");
    pub const PAGE: crate::Field<Self, crate::Text> = crate::Field::new("page");
    pub const SLIDE: crate::Field<Self, crate::Text> = crate::Field::new("slide");
    pub const WHITEBOARD: crate::Field<Self, crate::Text> = crate::Field::new("whiteboard");
}
