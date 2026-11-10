#[derive(Clone, Copy, Debug)]
pub struct DatabasePaginationMeta;

impl DatabasePaginationMeta {
    pub const CURSOR: crate::Field<Self, crate::Text> = crate::Field::new("cursor");
    pub const HAS_MORE: crate::Field<Self, crate::Boolean> = crate::Field::new("hasMore");
    pub const PAGE_SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("pageSize");
}
