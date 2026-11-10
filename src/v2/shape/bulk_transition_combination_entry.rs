#[derive(Clone, Copy, Debug)]
pub struct BulkTransitionCombinationEntry;

impl BulkTransitionCombinationEntry {
    pub const COMBINATION_ID: crate::Field<Self, crate::Text> = crate::Field::new("combinationId");

    pub const PERMISSIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::BulkTransitionDecodedPermission>>,
    > = crate::Field::new("permissions");

    pub const PRINCIPAL_COUNT: crate::Field<Self, crate::Integer> =
        crate::Field::new("principalCount");

    pub const PRINCIPAL_TYPES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("principalTypes");

    pub const SPACE_COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("spaceCount");
}
