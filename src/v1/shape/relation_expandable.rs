#[derive(Clone, Copy, Debug)]
pub struct RelationExpandable;

impl RelationExpandable {
    pub const RELATION_DATA: crate::Field<Self, crate::Text> = crate::Field::new("relationData");
    pub const SOURCE: crate::Field<Self, crate::Text> = crate::Field::new("source");
    pub const TARGET: crate::Field<Self, crate::Text> = crate::Field::new("target");
}
