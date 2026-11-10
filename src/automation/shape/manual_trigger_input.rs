#[derive(Clone, Copy, Debug)]
pub struct ManualTriggerInput;

impl ManualTriggerInput {
    pub const INPUT_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("inputType");
    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
