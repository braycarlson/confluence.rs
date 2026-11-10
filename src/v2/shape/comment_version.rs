#[derive(Clone, Copy, Debug)]
pub struct CommentVersion;

impl CommentVersion {
    pub const AUTHOR_ID: crate::Field<Self, crate::Text> = crate::Field::new("authorId");

    pub const COMMENT: crate::Field<Self, crate::Object<super::VersionedEntity>> =
        crate::Field::new("comment");

    pub const CREATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("createdAt");
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
    pub const MINOR_EDIT: crate::Field<Self, crate::Boolean> = crate::Field::new("minorEdit");
    pub const NUMBER: crate::Field<Self, crate::Integer> = crate::Field::new("number");
}
