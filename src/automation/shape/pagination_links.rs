#[derive(Clone, Copy, Debug)]
pub struct PaginationLinks;

impl PaginationLinks {
    pub const NEXT: crate::Field<Self, crate::Text> = crate::Field::new("next");
    pub const PREVIOUS: crate::Field<Self, crate::Text> = crate::Field::new("prev");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
