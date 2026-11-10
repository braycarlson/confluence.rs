#[derive(Clone, Copy, Debug)]
pub struct GetSpaceRoleModeReply;

impl GetSpaceRoleModeReply {
    pub const MODE: crate::Field<Self, crate::Text> = crate::Field::new("mode");
}
