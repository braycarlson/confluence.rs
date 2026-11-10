#[derive(Clone, Copy, Debug)]
pub struct DatabaseAiSummaryFieldSettings;

impl DatabaseAiSummaryFieldSettings {
    pub const AI_RUN_METADATA: crate::Field<
        Self,
        crate::Object<super::DatabaseAiSummaryRunMetadata>,
    > = crate::Field::new("aiRunMetadata");
}
