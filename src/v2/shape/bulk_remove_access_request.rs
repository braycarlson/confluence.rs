#[derive(Clone, Copy, Debug)]
pub struct BulkRemoveAccessRequest;

impl BulkRemoveAccessRequest {
    pub const PERMISSION_COMBINATION_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("permissionCombinationIds");

    pub const SPACE_SELECTION: crate::Field<
        Self,
        crate::Object<super::BulkTransitionSpaceSelection>,
    > = crate::Field::new("spaceSelection");
}
