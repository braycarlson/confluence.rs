#[derive(Clone, Copy, Debug)]
pub struct ComponentConfigCreateDTO;

impl ComponentConfigCreateDTO {
    pub const CHILDREN: crate::Field<Self, crate::List<crate::Object<Self>>> =
        crate::Field::new("children");

    pub const COMPONENT: crate::Field<Self, crate::Text> = crate::Field::new("component");

    pub const CONDITIONS: crate::Field<Self, crate::List<crate::Object<Self>>> =
        crate::Field::new("conditions");

    pub const CONDITION_PARENT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("conditionParentId");

    pub const CONNECTION_ID: crate::Field<Self, crate::Text> = crate::Field::new("connectionId");
    pub const PARENT_ID: crate::Field<Self, crate::Text> = crate::Field::new("parentId");

    pub const SCHEMA_VERSION: crate::Field<Self, crate::Integer> =
        crate::Field::new("schemaVersion");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
