#[derive(Clone, Copy, Debug)]
pub struct RestrictionStateResponse;

impl RestrictionStateResponse {
    pub const CREATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("createdAt");
    pub const CREATED_BY: crate::Field<Self, crate::Text> = crate::Field::new("createdBy");
    pub const CREATED_BY_AAID: crate::Field<Self, crate::Text> = crate::Field::new("createdByAaid");
    pub const STATE: crate::Field<Self, crate::Text> = crate::Field::new("state");
}
