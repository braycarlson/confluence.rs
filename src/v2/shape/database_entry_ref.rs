#[derive(Clone, Copy, Debug)]
pub struct DatabaseEntryRef;

impl DatabaseEntryRef {
    pub const DATABASE_CONTENT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("databaseContentId");

    pub const ENTRY_ID: crate::Field<Self, crate::Text> = crate::Field::new("entryId");
}
