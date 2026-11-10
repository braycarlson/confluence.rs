#[derive(Clone, Copy, Debug)]
pub struct AddContentRestriction;

impl AddContentRestriction {
    pub const OPERATION: crate::Field<Self, crate::Text> = crate::Field::new("operation");

    pub const RESTRICTIONS: crate::Field<
        Self,
        crate::Object<super::AddContentRestrictionRestrictions>,
    > = crate::Field::new("restrictions");
}
