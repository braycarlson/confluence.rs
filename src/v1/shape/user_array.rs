#[derive(Clone, Copy, Debug)]
pub struct UserArray;

impl UserArray {
    pub const LIMIT: crate::Field<Self, crate::Integer> = crate::Field::new("limit");
    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::User>>> =
        crate::Field::new("results");

    pub const SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("size");
    pub const SIZE_TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("totalSize");
    pub const START: crate::Field<Self, crate::Integer> = crate::Field::new("start");
}
