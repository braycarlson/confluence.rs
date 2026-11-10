#[derive(Clone, Copy, Debug)]
pub struct GlobalSpaceIdentifier;

impl GlobalSpaceIdentifier {
    pub const SPACE_IDENTIFIER: crate::Field<Self, crate::Text> =
        crate::Field::new("spaceIdentifier");
}
