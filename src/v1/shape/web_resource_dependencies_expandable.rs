#[derive(Clone, Copy, Debug)]
pub struct WebResourceDependenciesExpandable;

impl WebResourceDependenciesExpandable {
    pub const TAGS: crate::Field<Self, crate::Text> = crate::Field::new("tags");
    pub const URIS: crate::Field<Self, crate::Any> = crate::Field::new("uris");
}
