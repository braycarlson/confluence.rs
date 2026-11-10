#[derive(Clone, Copy, Debug)]
pub struct ManualTriggerInputPrompt;

impl ManualTriggerInputPrompt {
    pub const DEFAULT_VALUE: crate::Field<Self, crate::Any> = crate::Field::new("defaultValue");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const INPUT_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("inputType");
    pub const REQUIRED: crate::Field<Self, crate::Boolean> = crate::Field::new("required");
    pub const VARIABLE_NAME: crate::Field<Self, crate::Text> = crate::Field::new("variableName");
}
