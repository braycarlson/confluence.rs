#[derive(Clone, Copy, Debug)]
pub struct DatabasePageRef;

impl DatabasePageRef {
    pub const CONTENT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contentId");
}
