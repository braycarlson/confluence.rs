#[derive(Clone, Copy, Debug)]
pub struct GetUserPropertiesItem;

impl GetUserPropertiesItem {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
}
