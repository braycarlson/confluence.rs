#[derive(Clone, Copy, Debug)]
pub struct PermittedOperationsResponse;

impl PermittedOperationsResponse {
    pub const OPERATIONS: crate::Field<Self, crate::List<crate::Object<super::Operation>>> =
        crate::Field::new("operations");
}
