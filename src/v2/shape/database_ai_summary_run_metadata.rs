#[derive(Clone, Copy, Debug)]
pub struct DatabaseAiSummaryRunMetadata;

impl DatabaseAiSummaryRunMetadata {
    pub const AI_RUN_USER_ACCOUNT_ID_LAST: crate::Field<Self, crate::Text> =
        crate::Field::new("lastAiRunUserAccountId");
}
