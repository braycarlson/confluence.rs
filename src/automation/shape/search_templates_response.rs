#[derive(Clone, Copy, Debug)]
pub struct SearchTemplatesResponse;

impl SearchTemplatesResponse {
    pub const DATA: crate::Field<Self, crate::List<crate::Object<super::Template>>> =
        crate::Field::new("data");

    pub const LINKS: crate::Field<Self, crate::Object<super::PaginationLinks>> =
        crate::Field::new("links");
}
