#[derive(Clone, Copy, Debug)]
pub struct RuleUpdateRequestRule;

impl RuleUpdateRequestRule {
    pub const ACTOR: crate::Field<
        Self,
        crate::Object<super::RuleConfigResponsePropertiesRulePropertiesActor>,
    > = crate::Field::new("actor");

    pub const AUTHOR_ACCOUNT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("authorAccountId");

    pub const CAN_OTHER_RULE_TRIGGER: crate::Field<Self, crate::Boolean> =
        crate::Field::new("canOtherRuleTrigger");

    pub const COLLABORATORS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("collaborators");

    pub const COMPONENTS: crate::Field<
        Self,
        crate::List<crate::Object<super::ComponentConfigDTO>>,
    > = crate::Field::new("components");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const LABELS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("labels");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const NOTIFY_ON_ERROR: crate::Field<Self, crate::Text> = crate::Field::new("notifyOnError");

    pub const RULE_SCOPE_ARIS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("ruleScopeARIs");

    pub const STATE: crate::Field<Self, crate::Text> = crate::Field::new("state");

    pub const TRIGGER: crate::Field<Self, crate::Object<super::TriggerConfigDTO>> =
        crate::Field::new("trigger");

    pub const WRITE_ACCESS_TYPE: crate::Field<Self, crate::Text> =
        crate::Field::new("writeAccessType");
}
