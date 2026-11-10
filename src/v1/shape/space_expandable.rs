#[derive(Clone, Copy, Debug)]
pub struct SpaceExpandable;

impl SpaceExpandable {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const HISTORY: crate::Field<Self, crate::Text> = crate::Field::new("history");
    pub const HOMEPAGE: crate::Field<Self, crate::Text> = crate::Field::new("homepage");
    pub const ICON: crate::Field<Self, crate::Text> = crate::Field::new("icon");
    pub const IDENTIFIERS: crate::Field<Self, crate::Text> = crate::Field::new("identifiers");
    pub const LOOK_AND_FEEL: crate::Field<Self, crate::Text> = crate::Field::new("lookAndFeel");
    pub const METADATA: crate::Field<Self, crate::Text> = crate::Field::new("metadata");
    pub const OPERATIONS: crate::Field<Self, crate::Text> = crate::Field::new("operations");
    pub const PERMISSIONS: crate::Field<Self, crate::Text> = crate::Field::new("permissions");
    pub const ROLES: crate::Field<Self, crate::Text> = crate::Field::new("roles");
    pub const SETTINGS: crate::Field<Self, crate::Text> = crate::Field::new("settings");
    pub const THEME: crate::Field<Self, crate::Text> = crate::Field::new("theme");
    pub const TYPE_SETTINGS: crate::Field<Self, crate::Text> = crate::Field::new("typeSettings");
}
