#[derive(Clone, Copy, Debug)]
pub struct ContentIDToContentTypeResponse;

impl ContentIDToContentTypeResponse {
    pub const RESULTS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("results");
}
