#[derive(Clone, Copy, Debug)]
pub struct DatabaseBacklinkFieldSettings;

impl DatabaseBacklinkFieldSettings {
    pub const SOURCE_DATABASE_CONTENT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("sourceDatabaseContentId");
}
