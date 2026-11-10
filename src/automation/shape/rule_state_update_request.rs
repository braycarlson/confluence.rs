#[derive(Clone, Copy, Debug)]
pub struct RuleStateUpdateRequest;

impl RuleStateUpdateRequest {
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
