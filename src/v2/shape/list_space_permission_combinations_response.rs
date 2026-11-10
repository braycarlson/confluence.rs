#[derive(Clone, Copy, Debug)]
pub struct ListSpacePermissionCombinationsResponse;

impl ListSpacePermissionCombinationsResponse {
    pub const CURSOR: crate::Field<Self, crate::Text> = crate::Field::new("cursor");
    pub const GENERATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("generatedAt");

    pub const RESULTS: crate::Field<
        Self,
        crate::List<crate::Object<super::BulkTransitionCombinationEntry>>,
    > = crate::Field::new("results");
}
