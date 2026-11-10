#[derive(Clone, Copy, Debug)]
pub struct DatabaseHyperlinkItem;

impl DatabaseHyperlinkItem {
    pub const LABEL: crate::Field<Self, crate::Text> = crate::Field::new("label");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
