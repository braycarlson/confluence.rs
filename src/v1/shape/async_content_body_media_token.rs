#[derive(Clone, Copy, Debug)]
pub struct AsyncContentBodyMediaToken;

impl AsyncContentBodyMediaToken {
    pub const COLLECTION_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("collectionIds");

    pub const CONTENT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contentId");

    pub const EXPIRY_DATE_TIME: crate::Field<Self, crate::Text> =
        crate::Field::new("expiryDateTime");

    pub const FILE_IDS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("fileIds");
    pub const TOKEN: crate::Field<Self, crate::Text> = crate::Field::new("token");
}
