#[derive(Clone, Copy, Debug)]
pub struct RuleConfigResponseRuleActor;

impl RuleConfigResponseRuleActor {
    pub const ACTOR: crate::Field<Self, crate::Text> = crate::Field::new("actor");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
