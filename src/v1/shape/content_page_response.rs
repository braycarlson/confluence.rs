#[derive(Clone, Copy, Debug)]
pub struct ContentPageResponse;

impl ContentPageResponse {
    pub const LIMIT: crate::Field<Self, crate::Integer> = crate::Field::new("limit");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::Content>>> =
        crate::Field::new("results");

    pub const SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("size");
    pub const START: crate::Field<Self, crate::Integer> = crate::Field::new("start");
}
