#[derive(Clone, Copy, Debug)]
pub struct DatabaseJiraIssueValue;

impl DatabaseJiraIssueValue {
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const VALUE: crate::Field<Self, crate::List<crate::Object<super::DatabaseJiraIssueRef>>> =
        crate::Field::new("value");
}
