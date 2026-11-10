#[derive(Clone, Copy, Debug)]
pub struct Space;

impl Space {
    pub const ALIAS: crate::Field<Self, crate::Text> = crate::Field::new("alias");
    pub const ARI: crate::Field<Self, crate::Text> = crate::Field::new("ari");

    pub const DESCRIPTION: crate::Field<
        Self,
        crate::Object<super::SpaceDescriptionRepresentations>,
    > = crate::Field::new("description");

    pub const EXPANDABLE: crate::Field<Self, crate::Object<super::SpaceExpandable>> =
        crate::Field::new("_expandable");

    pub const HISTORY: crate::Field<Self, crate::Object<super::SpaceHistory>> =
        crate::Field::new("history");

    pub const HOMEPAGE: crate::Field<Self, crate::Object<super::Content>> =
        crate::Field::new("homepage");

    pub const ICON: crate::Field<Self, crate::Object<super::Icon>> = crate::Field::new("icon");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");

    pub const LOOK_AND_FEEL: crate::Field<Self, crate::Object<super::LookAndFeel>> =
        crate::Field::new("lookAndFeel");

    pub const METADATA: crate::Field<Self, crate::Object<super::SpaceMetadata>> =
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
