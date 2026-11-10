#[derive(Clone, Copy, Debug)]
pub struct SuperBatchWebResources;

impl SuperBatchWebResources {
    pub const EXPANDABLE: crate::Field<Self, crate::Map<crate::Any>> =
        crate::Field::new("_expandable");

    pub const METATAGS: crate::Field<Self, crate::Text> = crate::Field::new("metatags");

    pub const TAGS: crate::Field<Self, crate::Object<super::SuperBatchWebResourcesTags>> =
        crate::Field::new("tags");

    pub const URIS: crate::Field<Self, crate::Object<super::SuperBatchWebResourcesUris>> =
        crate::Field::new("uris");
}
