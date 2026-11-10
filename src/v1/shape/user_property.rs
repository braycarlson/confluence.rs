#[derive(Clone, Copy, Debug)]
pub struct UserProperty;

impl UserProperty {
    pub const CREATED_DATE: crate::Field<Self, crate::Text> = crate::Field::new("createdDate");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");

    pub const MODIFIED_DATE_LAST: crate::Field<Self, crate::Text> =
        crate::Field::new("lastModifiedDate");

    pub const VALUE: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("value");
}
