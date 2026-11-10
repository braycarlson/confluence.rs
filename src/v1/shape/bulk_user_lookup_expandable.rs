#[derive(Clone, Copy, Debug)]
pub struct BulkUserLookupExpandable;

impl BulkUserLookupExpandable {
    pub const DETAILS: crate::Field<Self, crate::Text> = crate::Field::new("details");
    pub const OPERATIONS: crate::Field<Self, crate::Text> = crate::Field::new("operations");
    pub const PERSONAL_SPACE: crate::Field<Self, crate::Text> = crate::Field::new("personalSpace");
}
