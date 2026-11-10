#[derive(Clone, Copy, Debug)]
pub struct ContentMetadata;

impl ContentMetadata {
    pub const COMMENT: crate::Field<Self, crate::Text> = crate::Field::new("comment");

    pub const CURRENTUSER: crate::Field<Self, crate::Object<super::ContentMetadataCurrentuser>> =
        crate::Field::new("currentuser");

    pub const EXPANDABLE: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("_expandable");

    pub const FRONTEND: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("frontend");
    pub const LABELS: crate::Field<Self, crate::Any> = crate::Field::new("labels");
    pub const MEDIA_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("mediaType");

    pub const PROPERTIES: crate::Field<Self, crate::Map<crate::Any>> =
        crate::Field::new("properties");
}
