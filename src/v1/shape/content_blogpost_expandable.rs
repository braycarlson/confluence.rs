#[derive(Clone, Copy, Debug)]
pub struct ContentBlogpostExpandable;

impl ContentBlogpostExpandable {
    pub const ANCESTORS: crate::Field<Self, crate::Text> = crate::Field::new("ancestors");
    pub const BODY: crate::Field<Self, crate::Text> = crate::Field::new("body");
    pub const CHILDREN: crate::Field<Self, crate::Text> = crate::Field::new("children");
    pub const CHILD_TYPES: crate::Field<Self, crate::Text> = crate::Field::new("childTypes");
    pub const CONTAINER: crate::Field<Self, crate::Text> = crate::Field::new("container");
    pub const DESCENDANTS: crate::Field<Self, crate::Text> = crate::Field::new("descendants");
    pub const DRAFT_VERSION: crate::Field<Self, crate::Text> = crate::Field::new("draftVersion");
    pub const EXTENSIONS: crate::Field<Self, crate::Text> = crate::Field::new("extensions");
    pub const HISTORY: crate::Field<Self, crate::Text> = crate::Field::new("history");

    pub const MACRO_RENDERED_OUTPUT: crate::Field<Self, crate::Text> =
        crate::Field::new("macroRenderedOutput");

    pub const METADATA: crate::Field<Self, crate::Text> = crate::Field::new("metadata");
    pub const OPERATIONS: crate::Field<Self, crate::Text> = crate::Field::new("operations");
    pub const RESTRICTIONS: crate::Field<Self, crate::Text> = crate::Field::new("restrictions");

    pub const SCHEDULE_PUBLISH_DATE: crate::Field<Self, crate::Text> =
        crate::Field::new("schedulePublishDate");

    pub const SCHEDULE_PUBLISH_INFORMATION: crate::Field<Self, crate::Text> =
        crate::Field::new("schedulePublishInfo");

    pub const SPACE: crate::Field<Self, crate::Text> = crate::Field::new("space");
    pub const VERSION: crate::Field<Self, crate::Text> = crate::Field::new("version");
}
