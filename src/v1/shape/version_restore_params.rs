#[derive(Clone, Copy, Debug)]
pub struct VersionRestoreParams;

impl VersionRestoreParams {
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
    pub const RESTORE_TITLE: crate::Field<Self, crate::Boolean> = crate::Field::new("restoreTitle");

    pub const VERSION_NUMBER: crate::Field<Self, crate::Integer> =
        crate::Field::new("versionNumber");
}
