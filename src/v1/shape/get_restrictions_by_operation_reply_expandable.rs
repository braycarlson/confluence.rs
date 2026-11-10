#[derive(Clone, Copy, Debug)]
pub struct GetRestrictionsByOperationReplyExpandable;

impl GetRestrictionsByOperationReplyExpandable {
    pub const READ: crate::Field<Self, crate::Text> = crate::Field::new("read");
    pub const UPDATE: crate::Field<Self, crate::Text> = crate::Field::new("update");
}
