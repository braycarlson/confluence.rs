#[derive(Clone, Copy, Debug)]
pub struct ErrorResponse;

impl ErrorResponse {
    pub const ERRORS: crate::Field<Self, crate::List<crate::Object<super::Error>>> =
        crate::Field::new("errors");
}
