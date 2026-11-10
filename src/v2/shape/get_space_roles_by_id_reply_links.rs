#[derive(Clone, Copy, Debug)]
pub struct GetSpaceRolesByIDReplyLinks;

impl GetSpaceRolesByIDReplyLinks {
    pub const BASE: crate::Field<Self, crate::Text> = crate::Field::new("base");
}
