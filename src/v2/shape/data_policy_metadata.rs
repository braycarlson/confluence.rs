#[derive(Clone, Copy, Debug)]
pub struct DataPolicyMetadata;

impl DataPolicyMetadata {
    pub const ANY_CONTENT_BLOCKED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("anyContentBlocked");
}
