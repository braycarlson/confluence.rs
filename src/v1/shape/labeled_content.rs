#[derive(Clone, Copy, Debug)]
pub struct LabeledContent;

impl LabeledContent {
    pub const CONTENT_ID: crate::Field<Self, crate::Integer> = crate::Field::new("contentId");
    pub const CONTENT_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("contentType");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
}
