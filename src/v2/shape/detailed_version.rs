#[derive(Clone, Copy, Debug)]
pub struct DetailedVersion;

impl DetailedVersion {
    pub const AUTHOR_ID: crate::Field<Self, crate::Text> = crate::Field::new("authorId");

    pub const COLLABORATORS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("collaborators");

    pub const CONTENT_TYPE_MODIFIED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("contentTypeModified");

    pub const CREATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("createdAt");
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
    pub const MINOR_EDIT: crate::Field<Self, crate::Boolean> = crate::Field::new("minorEdit");
    pub const NUMBER: crate::Field<Self, crate::Integer> = crate::Field::new("number");
    pub const VERSION_NEXT: crate::Field<Self, crate::Integer> = crate::Field::new("nextVersion");

    pub const VERSION_PREVIOUS: crate::Field<Self, crate::Integer> =
        crate::Field::new("prevVersion");
}
