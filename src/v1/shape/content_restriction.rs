#[derive(Clone, Copy, Debug)]
pub struct ContentRestriction;

impl ContentRestriction {
    pub const CONTENT: crate::Field<Self, crate::Object<super::Content>> =
        crate::Field::new("content");

    pub const EXPANDABLE: crate::Field<Self, crate::Object<super::ContentRestrictionExpandable>> =
        crate::Field::new("_expandable");

    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");
    pub const OPERATION: crate::Field<Self, crate::Text> = crate::Field::new("operation");

    pub const RESTRICTIONS: crate::Field<
        Self,
        crate::Object<super::ContentRestrictionRestrictions>,
    > = crate::Field::new("restrictions");
}
