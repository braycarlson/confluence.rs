#[derive(Clone, Copy, Debug)]
pub struct WebResourceDependencies;

impl WebResourceDependencies {
    pub const CONTEXTS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("contexts");

    pub const EXPANDABLE: crate::Field<
        Self,
        crate::Object<super::WebResourceDependenciesExpandable>,
    > = crate::Field::new("_expandable");

    pub const KEYS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("keys");

    pub const SUPERBATCH: crate::Field<Self, crate::Object<super::SuperBatchWebResources>> =
        crate::Field::new("superbatch");

    pub const TAGS: crate::Field<Self, crate::Object<super::WebResourceDependenciesTags>> =
        crate::Field::new("tags");

    pub const URIS: crate::Field<Self, crate::Object<super::WebResourceDependenciesUris>> =
        crate::Field::new("uris");
}
