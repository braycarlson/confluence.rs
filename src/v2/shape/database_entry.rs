#[derive(Clone, Copy, Debug)]
pub struct DatabaseEntry;

impl DatabaseEntry {
    pub const DATABASE_ID: crate::Field<Self, crate::Text> = crate::Field::new("databaseId");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const VALUES: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("values");
}
