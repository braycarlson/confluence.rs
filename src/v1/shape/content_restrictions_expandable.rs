#[derive(Clone, Copy, Debug)]
pub struct ContentRestrictionsExpandable;

impl ContentRestrictionsExpandable {
    pub const READ: crate::Field<Self, crate::Text> = crate::Field::new("read");
    pub const UPDATE: crate::Field<Self, crate::Text> = crate::Field::new("update");
}
