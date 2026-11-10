#[derive(Clone, Copy, Debug)]
pub struct Version;

impl Version {
    pub const BY_VALUE: crate::Field<Self, crate::Object<super::User>> = crate::Field::new("by");

    pub const COLLABORATORS: crate::Field<Self, crate::Object<super::UsersUserKeys>> =
        crate::Field::new("collaborators");

    pub const CONFIGURATION_REV: crate::Field<Self, crate::Text> = crate::Field::new("confRev");

    pub const CONTENT: crate::Field<Self, crate::Object<super::Content>> =
        crate::Field::new("content");

    pub const CONTENT_TYPE_MODIFIED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("contentTypeModified");

    pub const EXPANDABLE: crate::Field<Self, crate::Object<super::VersionExpandable>> =
        crate::Field::new("_expandable");

    pub const FRIENDLY_WHEN: crate::Field<Self, crate::Text> = crate::Field::new("friendlyWhen");
    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
    pub const MINOR_EDIT: crate::Field<Self, crate::Boolean> = crate::Field::new("minorEdit");

    pub const NCS_STEP_VERSION: crate::Field<Self, crate::Text> =
        crate::Field::new("ncsStepVersion");

    pub const NCS_STEP_VERSION_SOURCE: crate::Field<Self, crate::Text> =
        crate::Field::new("ncsStepVersionSource");

    pub const NUMBER: crate::Field<Self, crate::Integer> = crate::Field::new("number");
    pub const SYNCHRONIZE_REV: crate::Field<Self, crate::Text> = crate::Field::new("syncRev");

    pub const SYNCHRONIZE_REV_SOURCE: crate::Field<Self, crate::Text> =
        crate::Field::new("syncRevSource");

    pub const WHEN: crate::Field<Self, crate::Text> = crate::Field::new("when");
}
