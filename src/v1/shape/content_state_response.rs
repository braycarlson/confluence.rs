#[derive(Clone, Copy, Debug)]
pub struct ContentStateResponse;

impl ContentStateResponse {
    pub const CONTENT_STATE: crate::Field<Self, crate::Object<super::ContentState>> =
        crate::Field::new("contentState");

    pub const UPDATED_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastUpdated");
}
