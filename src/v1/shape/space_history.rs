#[derive(Clone, Copy, Debug)]
pub struct SpaceHistory;

impl SpaceHistory {
    pub const CREATED_BY: crate::Field<Self, crate::Object<super::User>> =
        crate::Field::new("createdBy");

    pub const CREATED_DATE: crate::Field<Self, crate::Text> = crate::Field::new("createdDate");
}
