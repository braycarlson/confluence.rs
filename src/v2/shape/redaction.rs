#[derive(Clone, Copy, Debug)]
pub struct Redaction;

impl Redaction {
    pub const REDACTIONS: crate::Field<Self, crate::List<crate::Object<super::RedactionPointer>>> =
        crate::Field::new("redactions");
}
