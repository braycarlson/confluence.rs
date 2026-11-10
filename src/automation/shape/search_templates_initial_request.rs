#[derive(Clone, Copy, Debug)]
pub struct SearchTemplatesInitialRequest;

impl SearchTemplatesInitialRequest {
    pub const CATEGORIES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("categories");

    pub const LIMIT: crate::Field<Self, crate::Number> = crate::Field::new("limit");
    pub const RULE_HOME: crate::Field<Self, crate::Text> = crate::Field::new("ruleHome");
}
