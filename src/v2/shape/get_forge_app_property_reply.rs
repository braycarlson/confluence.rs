#[derive(Clone, Copy, Debug)]
pub struct GetForgeAppPropertyReply;

impl GetForgeAppPropertyReply {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
