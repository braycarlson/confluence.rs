#[derive(Clone, Copy, Debug)]
pub struct ContentPropertyVersion;

impl ContentPropertyVersion {
    pub const CONTENT_TYPE_MODIFIED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("contentTypeModified");

    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
    pub const MINOR_EDIT: crate::Field<Self, crate::Boolean> = crate::Field::new("minorEdit");
    pub const NUMBER: crate::Field<Self, crate::Integer> = crate::Field::new("number");
    pub const WHEN: crate::Field<Self, crate::Text> = crate::Field::new("when");
}
