#[derive(Clone, Copy, Debug)]
pub struct Content;

impl Content {
    pub const ANCESTORS: crate::Field<Self, crate::List<crate::Object<Self>>> =
        crate::Field::new("ancestors");

    pub const ARI: crate::Field<Self, crate::Text> = crate::Field::new("ari");

    pub const BASE64_ENCODED_ARI: crate::Field<Self, crate::Text> =
        crate::Field::new("base64EncodedAri");

    pub const BODY: crate::Field<Self, crate::Object<super::ContentBodyRepresentations>> =
        crate::Field::new("body");

    pub const CHILDREN: crate::Field<Self, crate::Object<super::ContentChildren>> =
        crate::Field::new("children");

    pub const CHILD_TYPES: crate::Field<Self, crate::Object<super::ContentChildType>> =
        crate::Field::new("childTypes");

    pub const CONTAINER: crate::Field<Self, crate::Map<crate::Any>> =
        crate::Field::new("container");

    pub const DESCENDANTS: crate::Field<Self, crate::Object<super::ContentChildren>> =
        crate::Field::new("descendants");

    pub const EXPANDABLE: crate::Field<Self, crate::Object<super::ContentExpandable>> =
        crate::Field::new("_expandable");

    pub const EXTENSIONS: crate::Field<Self, crate::Any> = crate::Field::new("extensions");

    pub const HISTORY: crate::Field<Self, crate::Object<super::ContentHistory>> =
        crate::Field::new("history");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");

    pub const MACRO_RENDERED_OUTPUT: crate::Field<Self, crate::Map<crate::Any>> =
        crate::Field::new("macroRenderedOutput");

    pub const METADATA: crate::Field<Self, crate::Object<super::ContentMetadata>> =
        crate::Field::new("metadata");

    pub const OPERATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::OperationCheckResult>>,
    > = crate::Field::new("operations");

    pub const RESTRICTIONS: crate::Field<Self, crate::Object<super::ContentRestrictions>> =
        crate::Field::new("restrictions");

    pub const SPACE: crate::Field<Self, crate::Object<super::Space>> = crate::Field::new("space");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const VERSION: crate::Field<Self, crate::Object<super::Version>> =
        crate::Field::new("version");
}
