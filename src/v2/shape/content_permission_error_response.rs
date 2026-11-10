#[derive(Clone, Copy, Debug)]
pub struct ContentPermissionErrorResponse;

impl ContentPermissionErrorResponse {
    pub const ERRORS: crate::Field<
        Self,
        crate::List<crate::Object<super::ContentPermissionError>>,
    > = crate::Field::new("errors");
}
