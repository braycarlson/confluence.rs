#[derive(Clone, Copy, Debug)]
pub struct DatabaseJiraIssueDetailsFieldSettings;

impl DatabaseJiraIssueDetailsFieldSettings {
    pub const JIRA_FIELD_KEY: crate::Field<Self, crate::Text> = crate::Field::new("jiraFieldKey");
    pub const SOURCE_FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("sourceFieldId");
}
