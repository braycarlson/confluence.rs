#[derive(Clone, Copy, Debug)]
pub struct ContentCreateSpace;

impl ContentCreateSpace {
    pub const DESCRIPTION: crate::Field<Self, crate::Object<super::ContentCreateSpaceDescription>> =
        crate::Field::new("description");

    pub const HISTORY: crate::Field<Self, crate::Object<super::ContentCreateSpaceHistory>> =
        crate::Field::new("history");

    pub const HOMEPAGE: crate::Field<Self, crate::Object<super::Content>> =
        crate::Field::new("homepage");

    pub const ICON: crate::Field<Self, crate::Object<super::Icon>> = crate::Field::new("icon");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");

    pub const IDENTIFIERS: crate::Field<Self, crate::Object<super::GlobalSpaceIdentifier>> =
        crate::Field::new("identifiers");

    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("links");

    pub const LOOK_AND_FEEL: crate::Field<Self, crate::Object<super::LookAndFeel>> =
        crate::Field::new("lookAndFeel");

    pub const METADATA: crate::Field<Self, crate::Object<super::ContentCreateSpaceMetadata>> =
        crate::Field::new("metadata");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const OPERATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::OperationCheckResult>>,
    > = crate::Field::new("operations");

    pub const PERMISSIONS: crate::Field<Self, crate::List<crate::Object<super::SpacePermission>>> =
        crate::Field::new("permissions");

    pub const SETTINGS: crate::Field<Self, crate::Object<super::SpaceSettings>> =
        crate::Field::new("settings");

    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const THEME: crate::Field<Self, crate::Object<super::Theme>> = crate::Field::new("theme");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
