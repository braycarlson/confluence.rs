#[derive(Clone, Copy, Debug)]
pub struct ContainerSummary;

impl ContainerSummary {
    pub const DISPLAY_URL: crate::Field<Self, crate::Text> = crate::Field::new("displayUrl");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
}
