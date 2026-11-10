#[derive(Clone, Copy, Debug)]
pub struct ContentMetadataCurrentuserViewed;

impl ContentMetadataCurrentuserViewed {
    pub const FRIENDLY_SEEN_LAST: crate::Field<Self, crate::Text> =
        crate::Field::new("friendlyLastSeen");

    pub const SEEN_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastSeen");
}
