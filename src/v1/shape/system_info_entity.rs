#[derive(Clone, Copy, Debug)]
pub struct SystemInfoEntity;

impl SystemInfoEntity {
    pub const BASE_URL: crate::Field<Self, crate::Text> = crate::Field::new("baseUrl");
    pub const CLOUD_ID: crate::Field<Self, crate::Text> = crate::Field::new("cloudId");
    pub const COMMIT_HASH: crate::Field<Self, crate::Text> = crate::Field::new("commitHash");
    pub const DEFAULT_LOCALE: crate::Field<Self, crate::Text> = crate::Field::new("defaultLocale");

    pub const DEFAULT_TIME_ZONE: crate::Field<Self, crate::Text> =
        crate::Field::new("defaultTimeZone");

    pub const EDITION: crate::Field<Self, crate::Text> = crate::Field::new("edition");

    pub const FALLBACK_BASE_URL: crate::Field<Self, crate::Text> =
        crate::Field::new("fallbackBaseUrl");

    pub const MICROS_PERIMETER: crate::Field<Self, crate::Text> =
        crate::Field::new("microsPerimeter");

    pub const SITE_TITLE: crate::Field<Self, crate::Text> = crate::Field::new("siteTitle");
}
