#[derive(Clone, Copy, Debug)]
pub struct ContentMetadataCurrentuserLastcontributed;

impl ContentMetadataCurrentuserLastcontributed {
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const WHEN: crate::Field<Self, crate::Text> = crate::Field::new("when");
}
