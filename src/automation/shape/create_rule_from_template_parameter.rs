#[derive(Clone, Copy, Debug)]
pub struct CreateRuleFromTemplateParameter;

impl CreateRuleFromTemplateParameter {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const REQUIRED: crate::Field<Self, crate::Boolean> = crate::Field::new("required");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
