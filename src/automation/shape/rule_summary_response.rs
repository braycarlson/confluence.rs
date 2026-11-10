#[derive(Clone, Copy, Debug)]
pub struct RuleSummaryResponse;

impl RuleSummaryResponse {
    pub const DATA: crate::Field<Self, crate::List<crate::Object<super::RuleSummaryResponseData>>> =
        crate::Field::new("data");

    pub const LINKS: crate::Field<Self, crate::Object<super::PaginationLinks>> =
        crate::Field::new("links");
}
