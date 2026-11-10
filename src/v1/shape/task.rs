#[derive(Clone, Copy, Debug)]
pub struct Task;

impl Task {
    pub const ASSIGNEE: crate::Field<Self, crate::Text> = crate::Field::new("assignee");
    pub const BODY: crate::Field<Self, crate::Text> = crate::Field::new("body");
    pub const COMPLETE_DATE: crate::Field<Self, crate::Integer> = crate::Field::new("completeDate");
    pub const COMPLETE_USER: crate::Field<Self, crate::Text> = crate::Field::new("completeUser");
    pub const CONTENT_ID: crate::Field<Self, crate::Integer> = crate::Field::new("contentId");
    pub const CREATE_DATE: crate::Field<Self, crate::Integer> = crate::Field::new("createDate");
    pub const CREATOR: crate::Field<Self, crate::Text> = crate::Field::new("creator");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const DUE_DATE: crate::Field<Self, crate::Integer> = crate::Field::new("dueDate");
    pub const GLOBAL_ID: crate::Field<Self, crate::Integer> = crate::Field::new("globalId");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const UPDATE_DATE: crate::Field<Self, crate::Integer> = crate::Field::new("updateDate");
}
