#[derive(Clone, Copy, Debug)]
pub struct EmbeddedContent;

impl EmbeddedContent {
    pub const ENTITY: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("entity");
    pub const ENTITY_ID: crate::Field<Self, crate::Integer> = crate::Field::new("entityId");
    pub const ENTITY_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("entityType");
}
