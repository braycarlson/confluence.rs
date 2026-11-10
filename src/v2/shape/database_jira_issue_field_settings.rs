#[derive(Clone, Copy, Debug)]
pub struct DatabaseJiraIssueFieldSettings;

impl DatabaseJiraIssueFieldSettings {
    pub const CLOUD_ID: crate::Field<Self, crate::Text> = crate::Field::new("cloudId");
    pub const SERVER_ID: crate::Field<Self, crate::Text> = crate::Field::new("serverId");
}
