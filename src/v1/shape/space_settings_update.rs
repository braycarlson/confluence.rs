#[derive(Clone, Copy, Debug)]
pub struct SpaceSettingsUpdate;

impl SpaceSettingsUpdate {
    pub const CONTENT_MODE: crate::Field<Self, crate::Text> = crate::Field::new("contentMode");

    pub const ROUTE_OVERRIDE_ENABLED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("routeOverrideEnabled");
}
