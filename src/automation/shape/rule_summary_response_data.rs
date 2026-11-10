#[derive(Clone, Copy, Debug)]
pub struct RuleSummaryResponseData;

impl RuleSummaryResponseData {
    pub const ACTOR_ACCOUNT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("actorAccountId");

    pub const AUTHOR_ACCOUNT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("authorAccountId");

    pub const CREATED: crate::Field<Self, crate::Number> = crate::Field::new("created");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const LABELS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("labels");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const RULE_SCOPE_ARIS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("ruleScopeARIs");

    pub const STATE: crate::Field<Self, crate::Text> = crate::Field::new("state");
    pub const UPDATED: crate::Field<Self, crate::Number> = crate::Field::new("updated");
    pub const UUID: crate::Field<Self, crate::Text> = crate::Field::new("uuid");
}
