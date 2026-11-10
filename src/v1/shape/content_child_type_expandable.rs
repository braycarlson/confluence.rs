#[derive(Clone, Copy, Debug)]
pub struct ContentChildTypeExpandable;

impl ContentChildTypeExpandable {
    pub const ALL: crate::Field<Self, crate::Text> = crate::Field::new("all");
    pub const ATTACHMENT: crate::Field<Self, crate::Text> = crate::Field::new("attachment");
    pub const COMMENT: crate::Field<Self, crate::Text> = crate::Field::new("comment");
    pub const DATABASE: crate::Field<Self, crate::Text> = crate::Field::new("database");
    pub const EMBED: crate::Field<Self, crate::Text> = crate::Field::new("embed");
    pub const FOLDER: crate::Field<Self, crate::Text> = crate::Field::new("folder");
    pub const PAGE: crate::Field<Self, crate::Text> = crate::Field::new("page");
    pub const WHITEBOARD: crate::Field<Self, crate::Text> = crate::Field::new("whiteboard");
}
