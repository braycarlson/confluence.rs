#[derive(Clone, Copy, Debug)]
pub struct RuleScopeUpdateRequest;

impl RuleScopeUpdateRequest {
    pub const RULE_SCOPE_ARIS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("ruleScopeARIs");
}
