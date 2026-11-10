#[derive(Clone, Copy, Debug)]
pub struct OptionalFieldLinks;

impl OptionalFieldLinks {
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
