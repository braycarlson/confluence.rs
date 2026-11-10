#[derive(Clone, Copy, Debug)]
pub struct ContentPropertyExpandable;

impl ContentPropertyExpandable {
    pub const ADDITIONAL_PROPERTIES: crate::Field<Self, crate::Text> =
        crate::Field::new("additionalProperties");

    pub const CONTENT: crate::Field<Self, crate::Text> = crate::Field::new("content");
}
