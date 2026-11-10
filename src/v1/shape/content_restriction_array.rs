#[derive(Clone, Copy, Debug)]
pub struct ContentRestrictionArray;

impl ContentRestrictionArray {
    pub const LIMIT: crate::Field<Self, crate::Integer> = crate::Field::new("limit");
    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");

    pub const RESTRICTIONS_HASH: crate::Field<Self, crate::Text> =
        crate::Field::new("restrictionsHash");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::ContentRestriction>>> =
        crate::Field::new("results");

    pub const SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("size");
    pub const START: crate::Field<Self, crate::Integer> = crate::Field::new("start");
}
