#[derive(Clone, Copy, Debug)]
pub struct WebResourceDependenciesUris;

impl WebResourceDependenciesUris {
    pub const ALL: crate::Field<Self, crate::Any> = crate::Field::new("all");
    pub const CSS: crate::Field<Self, crate::Any> = crate::Field::new("css");

    pub const EXPANDABLE: crate::Field<
        Self,
        crate::Object<super::WebResourceDependenciesUrisExpandable>,
    > = crate::Field::new("_expandable");

    pub const JS_VALUE: crate::Field<Self, crate::Any> = crate::Field::new("js");
}
