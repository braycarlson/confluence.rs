#[derive(Clone, Copy, Debug)]
pub struct SearchResult;

impl SearchResult {
    pub const BREADCRUMBS: crate::Field<Self, crate::List<crate::Object<super::Breadcrumb>>> =
        crate::Field::new("breadcrumbs");

    pub const CONTENT: crate::Field<Self, crate::Object<super::Content>> =
        crate::Field::new("content");

    pub const ENTITY_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("entityType");
    pub const EXCERPT: crate::Field<Self, crate::Text> = crate::Field::new("excerpt");

    pub const FRIENDLY_MODIFIED_LAST: crate::Field<Self, crate::Text> =
        crate::Field::new("friendlyLastModified");

    pub const ICON_CSS_CLASS: crate::Field<Self, crate::Text> = crate::Field::new("iconCssClass");
    pub const MODIFIED_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastModified");

    pub const RESULT_GLOBAL_CONTAINER: crate::Field<Self, crate::Object<super::ContainerSummary>> =
        crate::Field::new("resultGlobalContainer");

    pub const RESULT_PARENT_CONTAINER: crate::Field<Self, crate::Object<super::ContainerSummary>> =
        crate::Field::new("resultParentContainer");

    pub const SCORE: crate::Field<Self, crate::Number> = crate::Field::new("score");
    pub const SPACE: crate::Field<Self, crate::Object<super::Space>> = crate::Field::new("space");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
    pub const USER: crate::Field<Self, crate::Object<super::User>> = crate::Field::new("user");
}
