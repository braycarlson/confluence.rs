#[derive(Clone, Copy, Debug)]
pub struct RuleScope;

impl RuleScope {
    pub const RESOURCES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("resources");
}
