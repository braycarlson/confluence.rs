#[derive(Clone, Copy, Debug)]
pub struct SpaceDescription;

impl SpaceDescription {
    pub const EMBEDDED_CONTENT: crate::Field<Self, crate::List<crate::Any>> =
        crate::Field::new("embeddedContent");

    pub const REPRESENTATION: crate::Field<Self, crate::Text> = crate::Field::new("representation");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
