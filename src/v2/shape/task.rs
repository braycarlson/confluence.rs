#[derive(Clone, Copy, Debug)]
pub struct Task;

impl Task {
    pub const ASSIGNED_TO: crate::Field<Self, crate::Text> = crate::Field::new("assignedTo");
    pub const BLOG_POST_ID: crate::Field<Self, crate::Text> = crate::Field::new("blogPostId");

    pub const BODY: crate::Field<Self, crate::Object<super::TaskBodySingle>> =
        crate::Field::new("body");

    pub const COMPLETED_AT: crate::Field<Self, crate::Text> = crate::Field::new("completedAt");
    pub const COMPLETED_BY: crate::Field<Self, crate::Text> = crate::Field::new("completedBy");
    pub const CREATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("createdAt");
    pub const CREATED_BY: crate::Field<Self, crate::Text> = crate::Field::new("createdBy");
    pub const DUE_AT: crate::Field<Self, crate::Text> = crate::Field::new("dueAt");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const LOCAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("localId");
    pub const PAGE_ID: crate::Field<Self, crate::Text> = crate::Field::new("pageId");
    pub const SPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("spaceId");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const UPDATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("updatedAt");
}
