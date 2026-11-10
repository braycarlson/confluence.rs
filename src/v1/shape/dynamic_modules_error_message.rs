#[derive(Clone, Copy, Debug)]
pub struct DynamicModulesErrorMessage;

impl DynamicModulesErrorMessage {
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
}
