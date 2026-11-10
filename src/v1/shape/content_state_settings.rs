#[derive(Clone, Copy, Debug)]
pub struct ContentStateSettings;

impl ContentStateSettings {
    pub const CONTENT_STATES_ALLOWED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("contentStatesAllowed");

    pub const CUSTOM_CONTENT_STATES_ALLOWED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("customContentStatesAllowed");

    pub const SPACE_CONTENT_STATES: crate::Field<
        Self,
        crate::List<crate::Object<super::ContentState>>,
    > = crate::Field::new("spaceContentStates");

    pub const SPACE_CONTENT_STATES_ALLOWED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("spaceContentStatesAllowed");
}
