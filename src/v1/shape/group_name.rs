#[derive(Clone, Copy, Debug)]
pub struct GroupName;

impl GroupName {
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
