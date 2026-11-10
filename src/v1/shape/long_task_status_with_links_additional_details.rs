#[derive(Clone, Copy, Debug)]
pub struct LongTaskStatusWithLinksAdditionalDetails;

impl LongTaskStatusWithLinksAdditionalDetails {
    pub const ADDITIONAL_PROPERTIES: crate::Field<Self, crate::Text> =
        crate::Field::new("additionalProperties");

    pub const DESTINATION_ID: crate::Field<Self, crate::Text> = crate::Field::new("destinationId");

    pub const DESTINATION_URL: crate::Field<Self, crate::Text> =
        crate::Field::new("destinationUrl");

    pub const PAGE_NEED_TO_COPY_TOTAL: crate::Field<Self, crate::Integer> =
        crate::Field::new("totalPageNeedToCopy");
}
