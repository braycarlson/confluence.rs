#[derive(Clone, Copy, Debug)]
pub struct ChildrenResponse;

impl ChildrenResponse {
    pub const CHILD_POSITION: crate::Field<Self, crate::Integer> =
        crate::Field::new("childPosition");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const SPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("spaceId");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
