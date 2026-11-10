#[derive(Clone, Copy, Debug)]
pub struct CustomContentVersion;

impl CustomContentVersion {
    pub const AUTHOR_ID: crate::Field<Self, crate::Text> = crate::Field::new("authorId");
    pub const CREATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("createdAt");

    pub const CUSTOM: crate::Field<Self, crate::Object<super::VersionedEntity>> =
        crate::Field::new("custom");

    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
    pub const MINOR_EDIT: crate::Field<Self, crate::Boolean> = crate::Field::new("minorEdit");
    pub const NUMBER: crate::Field<Self, crate::Integer> = crate::Field::new("number");
}
