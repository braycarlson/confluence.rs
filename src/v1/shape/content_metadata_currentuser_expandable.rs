#[derive(Clone, Copy, Debug)]
pub struct ContentMetadataCurrentuserExpandable;

impl ContentMetadataCurrentuserExpandable {
    pub const FAVOURITED: crate::Field<Self, crate::Text> = crate::Field::new("favourited");

    pub const LASTCONTRIBUTED: crate::Field<Self, crate::Text> =
        crate::Field::new("lastcontributed");

    pub const LASTMODIFIED: crate::Field<Self, crate::Text> = crate::Field::new("lastmodified");
    pub const SCHEDULED: crate::Field<Self, crate::Text> = crate::Field::new("scheduled");
    pub const VIEWED: crate::Field<Self, crate::Text> = crate::Field::new("viewed");
}
