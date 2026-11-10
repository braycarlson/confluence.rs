#[derive(Clone, Copy, Debug)]
pub struct SpacePropertyVersion;

impl SpacePropertyVersion {
    pub const AUTHOR_ID: crate::Field<Self, crate::Text> = crate::Field::new("authorId");
    pub const CREATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("createdAt");
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
    pub const NUMBER: crate::Field<Self, crate::Integer> = crate::Field::new("number");
}
