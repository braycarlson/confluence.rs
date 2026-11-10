#[derive(Clone, Copy, Debug)]
pub struct CQLPersonalDataConvertedQueries;

impl CQLPersonalDataConvertedQueries {
    pub const QUERY_STRINGS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("queryStrings");
}
