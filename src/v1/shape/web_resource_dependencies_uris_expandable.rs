#[derive(Clone, Copy, Debug)]
pub struct WebResourceDependenciesUrisExpandable;

impl WebResourceDependenciesUrisExpandable {
    pub const CSS: crate::Field<Self, crate::Any> = crate::Field::new("css");
    pub const JS_VALUE: crate::Field<Self, crate::Any> = crate::Field::new("js");
}
