#[derive(Clone, Copy, Debug)]
pub struct SearchManualRuleResponse;

impl SearchManualRuleResponse {
    pub const DATA: crate::Field<Self, crate::List<crate::Object<super::ManualTriggerRule>>> =
        crate::Field::new("data");

    pub const LINKS: crate::Field<Self, crate::Object<super::PaginationLinks>> =
        crate::Field::new("links");
}
