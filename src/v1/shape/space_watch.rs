#[derive(Clone, Copy, Debug)]
pub struct SpaceWatch;

impl SpaceWatch {
    pub const LABEL_NAME: crate::Field<Self, crate::Text> = crate::Field::new("labelName");
    pub const PREFIX: crate::Field<Self, crate::Text> = crate::Field::new("prefix");
    pub const SPACE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("spaceKey");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const WATCHER: crate::Field<Self, crate::Object<super::WatchUser>> =
        crate::Field::new("watcher");
}
