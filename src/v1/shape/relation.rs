#[derive(Clone, Copy, Debug)]
pub struct Relation;

impl Relation {
    pub const EXPANDABLE: crate::Field<Self, crate::Object<super::RelationExpandable>> =
        crate::Field::new("_expandable");

    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const RELATION_DATA: crate::Field<Self, crate::Object<super::RelationData>> =
        crate::Field::new("relationData");

    pub const SOURCE: crate::Field<Self, crate::Any> = crate::Field::new("source");
    pub const TARGET: crate::Field<Self, crate::Any> = crate::Field::new("target");
}
