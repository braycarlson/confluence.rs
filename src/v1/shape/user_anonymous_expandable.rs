#[derive(Clone, Copy, Debug)]
pub struct UserAnonymousExpandable;

impl UserAnonymousExpandable {
    pub const OPERATIONS: crate::Field<Self, crate::Text> = crate::Field::new("operations");
}
