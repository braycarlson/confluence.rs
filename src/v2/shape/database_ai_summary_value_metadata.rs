#[derive(Clone, Copy, Debug)]
pub struct DatabaseAiSummaryValueMetadata;

impl DatabaseAiSummaryValueMetadata {
    pub const UPDATED_AT_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastUpdatedAt");
    pub const UPDATED_BY_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastUpdatedBy");

    pub const UPDATED_USER_ACCOUNT_ID_LAST: crate::Field<Self, crate::Text> =
        crate::Field::new("lastUpdatedUserAccountId");
}
