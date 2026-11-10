#[derive(Clone, Copy, Debug)]
pub struct TriggerConfigDTO;

impl TriggerConfigDTO {
    pub const COMPONENT: crate::Field<Self, crate::Text> = crate::Field::new("component");

    pub const CONDITIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::ComponentConfigDTO>>,
    > = crate::Field::new("conditions");

    pub const CONNECTION_ID: crate::Field<Self, crate::Text> = crate::Field::new("connectionId");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const SCHEMA_VERSION: crate::Field<Self, crate::Integer> =
        crate::Field::new("schemaVersion");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
