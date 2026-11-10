#[derive(Clone, Copy, Debug)]
pub struct VersionExpandable;

impl VersionExpandable {
    pub const COLLABORATORS: crate::Field<Self, crate::Text> = crate::Field::new("collaborators");
    pub const CONTENT: crate::Field<Self, crate::Text> = crate::Field::new("content");
}
