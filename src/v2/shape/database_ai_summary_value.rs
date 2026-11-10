#[derive(Clone, Copy, Debug)]
pub struct DatabaseAiSummaryValue;

impl DatabaseAiSummaryValue {
    pub const METADATA: crate::Field<Self, crate::Object<super::DatabaseAiSummaryValueMetadata>> =
        crate::Field::new("metadata");

    pub const PLAIN: crate::Field<Self, crate::Text> = crate::Field::new("plain");

    pub const RICH: crate::Field<Self, crate::Object<super::DatabaseADFDocument>> =
        crate::Field::new("rich");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
