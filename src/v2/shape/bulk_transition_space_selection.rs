#[derive(Clone, Copy, Debug)]
pub struct BulkTransitionSpaceSelection;

impl BulkTransitionSpaceSelection {
    pub const SELECTED_SPACES: crate::Field<
        Self,
        crate::List<crate::Object<super::BulkTransitionSpaceTarget>>,
    > = crate::Field::new("selectedSpaces");

    pub const SPACE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("spaceType");
}
