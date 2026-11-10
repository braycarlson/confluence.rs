#[derive(Clone, Copy, Debug)]
pub struct RedactionResponse;

impl RedactionResponse {
    pub const BODY: crate::Field<Self, crate::Object<super::RedactionSectionResponse>> =
        crate::Field::new("body");

    pub const TITLE: crate::Field<Self, crate::Object<super::RedactionSectionResponse>> =
        crate::Field::new("title");
}
