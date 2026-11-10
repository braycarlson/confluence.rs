#[derive(Clone, Copy, Debug)]
pub struct AvailableContentStates;

impl AvailableContentStates {
    pub const CUSTOM_CONTENT_STATES: crate::Field<
        Self,
        crate::List<crate::Object<super::ContentState>>,
    > = crate::Field::new("customContentStates");

    pub const SPACE_CONTENT_STATES: crate::Field<
        Self,
        crate::List<crate::Object<super::ContentState>>,
    > = crate::Field::new("spaceContentStates");
}
