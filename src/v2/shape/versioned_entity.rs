#[derive(Clone, Copy, Debug)]
pub struct VersionedEntity;

impl VersionedEntity {
    pub const BODY: crate::Field<Self, crate::Object<super::BodyBulk>> = crate::Field::new("body");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
}
