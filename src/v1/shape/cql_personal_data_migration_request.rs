#[derive(Clone, Copy, Debug)]
pub struct CQLPersonalDataMigrationRequest;

impl CQLPersonalDataMigrationRequest {
    pub const QUERY_STRINGS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("queryStrings");
}
