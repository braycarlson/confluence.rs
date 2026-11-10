#[derive(Clone, Copy, Debug)]
pub struct AttachmentPropertiesUpdateBody;

impl AttachmentPropertiesUpdateBody {
    pub const CONTAINER: crate::Field<Self, crate::Map<crate::Any>> =
        crate::Field::new("container");

    pub const EXTENSIONS: crate::Field<Self, crate::Any> = crate::Field::new("extensions");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const METADATA: crate::Field<
        Self,
        crate::Object<super::AttachmentPropertiesUpdateBodyMetadata>,
    > = crate::Field::new("metadata");

    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const VERSION: crate::Field<Self, crate::Object<super::Version>> =
        crate::Field::new("version");
}
