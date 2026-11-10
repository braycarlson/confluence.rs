#[derive(Clone, Copy, Debug)]
pub struct InvokeManualRuleRequest;

impl InvokeManualRuleRequest {
    pub const OBJECTS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("objects");

    pub const USER_INPUTS: crate::Field<
        Self,
        crate::Map<crate::Object<super::ManualTriggerInput>>,
    > = crate::Field::new("userInputs");
}
