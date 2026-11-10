#[derive(Clone, Copy, Debug)]
pub struct AttachmentUpdate;

impl AttachmentUpdate {
    pub const CONTAINER: crate::Field<Self, crate::Object<super::AttachmentUpdateContainer>> =
        crate::Field::new("container");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const METADATA: crate::Field<Self, crate::Object<super::AttachmentUpdateMetadata>> =
        crate::Field::new("metadata");

    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const VERSION: crate::Field<Self, crate::Object<super::AttachmentUpdateVersion>> =
        crate::Field::new("version");
}
