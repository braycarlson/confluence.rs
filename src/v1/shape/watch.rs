#[derive(Clone, Copy, Debug)]
pub struct Watch;

impl Watch {
    pub const CONTENT_ID: crate::Field<Self, crate::Integer> = crate::Field::new("contentId");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const WATCHER: crate::Field<Self, crate::Object<super::WatchUser>> =
        crate::Field::new("watcher");
}
