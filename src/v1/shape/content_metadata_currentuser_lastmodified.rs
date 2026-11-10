#[derive(Clone, Copy, Debug)]
pub struct ContentMetadataCurrentuserLastmodified;

impl ContentMetadataCurrentuserLastmodified {
    pub const FRIENDLY_MODIFIED_LAST: crate::Field<Self, crate::Text> =
        crate::Field::new("friendlyLastModified");

    pub const VERSION: crate::Field<Self, crate::Object<super::Version>> =
        crate::Field::new("version");
}
