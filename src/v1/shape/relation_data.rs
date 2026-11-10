#[derive(Clone, Copy, Debug)]
pub struct RelationData;

impl RelationData {
    pub const CREATED_BY: crate::Field<Self, crate::Object<super::User>> =
        crate::Field::new("createdBy");

    pub const CREATED_DATE: crate::Field<Self, crate::Text> = crate::Field::new("createdDate");

    pub const FRIENDLY_CREATED_DATE: crate::Field<Self, crate::Text> =
        crate::Field::new("friendlyCreatedDate");
}
