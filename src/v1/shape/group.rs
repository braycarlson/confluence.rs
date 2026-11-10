#[derive(Clone, Copy, Debug)]
pub struct Group;

impl Group {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");
    pub const MANAGED_BY: crate::Field<Self, crate::Text> = crate::Field::new("managedBy");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const RESOURCE_ARI: crate::Field<Self, crate::Text> = crate::Field::new("resourceAri");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const USAGE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("usageType");
}
