#[derive(Clone, Copy, Debug)]
pub struct AffectedObject;

impl AffectedObject {
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const OBJECT_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("objectType");
}
