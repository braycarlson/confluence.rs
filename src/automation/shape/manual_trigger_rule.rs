#[derive(Clone, Copy, Debug)]
pub struct ManualTriggerRule;

impl ManualTriggerRule {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const USER_INPUTS: crate::Field<
        Self,
        crate::List<crate::Object<super::ManualTriggerInputPrompt>>,
    > = crate::Field::new("userInputs");
}
