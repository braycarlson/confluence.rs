#[derive(Clone, Copy, Debug)]
pub struct DatabaseEntrySingleResponse;

impl DatabaseEntrySingleResponse {
    pub const DATABASE_ID: crate::Field<Self, crate::Text> = crate::Field::new("databaseId");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LINKS: crate::Field<Self, crate::Object<super::DatabasesPublicAPILinks>> =
        crate::Field::new("_links");

    pub const VALUES: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("values");
}
