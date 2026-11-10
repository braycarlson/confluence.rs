#[derive(Clone, Copy, Debug)]
pub struct CreateRuleFromTemplateParameterValue;

impl CreateRuleFromTemplateParameterValue {
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
