#[derive(Clone, Copy, Debug)]
pub struct GetViewersReply;

impl GetViewersReply {
    pub const COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("count");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
}
