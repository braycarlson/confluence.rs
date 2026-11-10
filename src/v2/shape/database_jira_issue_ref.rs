#[derive(Clone, Copy, Debug)]
pub struct DatabaseJiraIssueRef;

impl DatabaseJiraIssueRef {
    pub const CLOUD_ID: crate::Field<Self, crate::Text> = crate::Field::new("cloudId");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const SERVER_ID: crate::Field<Self, crate::Text> = crate::Field::new("serverId");
}
