#[derive(Clone, Copy, Debug)]
pub struct SpaceSettings;

impl SpaceSettings {
    pub const CONTENT_MODE: crate::Field<Self, crate::Text> = crate::Field::new("contentMode");

    pub const EDITOR: crate::Field<Self, crate::Object<super::SpaceSettingsEditor>> =
        crate::Field::new("editor");

    pub const IS_PDF_EXPORT_NO_CODE_STYLING_OPTED_IN: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isPdfExportNoCodeStylingOptedIn");

    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");

    pub const ROUTE_OVERRIDE_ENABLED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("routeOverrideEnabled");

    pub const SPACE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("spaceKey");
}
