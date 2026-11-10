#[derive(Clone, Copy, Debug)]
pub struct SearchPageResponseSearchResult;

impl SearchPageResponseSearchResult {
    pub const ARCHIVED_RESULT_COUNT: crate::Field<Self, crate::Integer> =
        crate::Field::new("archivedResultCount");

    pub const CQL_QUERY: crate::Field<Self, crate::Text> = crate::Field::new("cqlQuery");
    pub const LIMIT: crate::Field<Self, crate::Integer> = crate::Field::new("limit");
    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::SearchResult>>> =
        crate::Field::new("results");

    pub const SEARCH_DURATION: crate::Field<Self, crate::Integer> =
        crate::Field::new("searchDuration");

    pub const SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("size");
    pub const SIZE_TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("totalSize");
    pub const START: crate::Field<Self, crate::Integer> = crate::Field::new("start");
}
