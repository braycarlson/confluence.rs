#[derive(Clone, Copy, Debug)]
pub struct GetForgeAppPropertiesReplyResults;

impl GetForgeAppPropertiesReplyResults {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
