#[derive(Clone, Copy, Debug)]
pub struct DatabasesPublicAPILinks;

impl DatabasesPublicAPILinks {
    pub const BASE: crate::Field<Self, crate::Text> = crate::Field::new("base");
    pub const NEXT: crate::Field<Self, crate::Text> = crate::Field::new("next");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
