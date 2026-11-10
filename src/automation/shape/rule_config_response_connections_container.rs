#[derive(Clone, Copy, Debug)]
pub struct RuleConfigResponseConnectionsContainer;

impl RuleConfigResponseConnectionsContainer {
    pub const CONTAINER_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("containerType");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
