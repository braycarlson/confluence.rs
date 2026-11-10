#[derive(Clone, Copy, Debug)]
pub struct CopyPageHierarchyTitleOptions;

impl CopyPageHierarchyTitleOptions {
    pub const PREFIX: crate::Field<Self, crate::Text> = crate::Field::new("prefix");
    pub const REPLACE: crate::Field<Self, crate::Text> = crate::Field::new("replace");
    pub const SEARCH: crate::Field<Self, crate::Text> = crate::Field::new("search");
}
