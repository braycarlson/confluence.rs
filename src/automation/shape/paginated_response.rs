#[derive(Clone, Copy, Debug)]
pub struct PaginatedResponse;

impl PaginatedResponse {
    pub const DATA: crate::Field<Self, crate::List<crate::Any>> = crate::Field::new("data");

    pub const LINKS: crate::Field<Self, crate::Object<super::PaginationLinks>> =
        crate::Field::new("links");
}
