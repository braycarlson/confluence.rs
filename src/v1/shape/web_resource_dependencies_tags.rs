#[derive(Clone, Copy, Debug)]
pub struct WebResourceDependenciesTags;

impl WebResourceDependenciesTags {
    pub const ALL: crate::Field<Self, crate::Text> = crate::Field::new("all");
    pub const CSS: crate::Field<Self, crate::Text> = crate::Field::new("css");
    pub const DATA: crate::Field<Self, crate::Text> = crate::Field::new("data");

    pub const EXPANDABLE: crate::Field<Self, crate::Map<crate::Any>> =
        crate::Field::new("_expandable");

    pub const JS_VALUE: crate::Field<Self, crate::Text> = crate::Field::new("js");
}
