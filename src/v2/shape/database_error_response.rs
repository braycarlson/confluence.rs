#[derive(Clone, Copy, Debug)]
pub struct DatabaseErrorResponse;

impl DatabaseErrorResponse {
    pub const ERRORS: crate::Field<Self, crate::List<crate::Object<super::DatabaseError>>> =
        crate::Field::new("errors");
}
