#[derive(Clone, Copy, Debug)]
pub struct ContentRestrictionUpdate;

impl ContentRestrictionUpdate {
    pub const CONTENT: crate::Field<Self, crate::Object<super::ContentRestrictionUpdateContent>> =
        crate::Field::new("content");

    pub const OPERATION: crate::Field<Self, crate::Text> = crate::Field::new("operation");

    pub const RESTRICTIONS: crate::Field<
        Self,
        crate::Object<super::ContentRestrictionUpdateRestrictions>,
    > = crate::Field::new("restrictions");
}
