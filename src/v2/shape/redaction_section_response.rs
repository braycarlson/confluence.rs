#[derive(Clone, Copy, Debug)]
pub struct RedactionSectionResponse;

impl RedactionSectionResponse {
    pub const REDACTIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::RedactionPointerResponse>>,
    > = crate::Field::new("redactions");
}
