#[derive(Clone, Copy, Debug)]
pub struct DatabaseViewListResponse;

impl DatabaseViewListResponse {
    pub const LINKS: crate::Field<Self, crate::Object<super::DatabasesPublicAPILinks>> =
        crate::Field::new("_links");

    pub const META: crate::Field<Self, crate::Object<super::DatabasePaginationMeta>> =
        crate::Field::new("meta");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::DatabaseView>>> =
        crate::Field::new("results");
}
