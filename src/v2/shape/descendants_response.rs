#[derive(Clone, Copy, Debug)]
pub struct DescendantsResponse;

impl DescendantsResponse {
    pub const CHILD_POSITION: crate::Field<Self, crate::Integer> =
        crate::Field::new("childPosition");

    pub const DEPTH: crate::Field<Self, crate::Integer> = crate::Field::new("depth");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const MODIFIED_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastModified");
    pub const PARENT_ID: crate::Field<Self, crate::Text> = crate::Field::new("parentId");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
