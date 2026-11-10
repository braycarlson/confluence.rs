#[derive(Clone, Copy, Debug)]
pub struct DataPolicySpaceDataPolicy;

impl DataPolicySpaceDataPolicy {
    pub const ANY_CONTENT_BLOCKED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("anyContentBlocked");
}
