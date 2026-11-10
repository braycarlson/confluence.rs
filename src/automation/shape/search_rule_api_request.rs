#[derive(Clone, Copy, Debug)]
pub struct SearchRuleAPIRequest;

impl SearchRuleAPIRequest {
    pub const AUTHOR: crate::Field<Self, crate::Text> = crate::Field::new("author");
    pub const CURSOR: crate::Field<Self, crate::Text> = crate::Field::new("cursor");
    pub const LIMIT: crate::Field<Self, crate::Number> = crate::Field::new("limit");
    pub const SCOPE: crate::Field<Self, crate::Text> = crate::Field::new("scope");
    pub const STATE: crate::Field<Self, crate::Text> = crate::Field::new("state");
    pub const TRIGGER: crate::Field<Self, crate::Text> = crate::Field::new("trigger");
}
