#[derive(Clone, Copy, Debug)]
pub struct LabelCreate;

impl LabelCreate {
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const PREFIX: crate::Field<Self, crate::Text> = crate::Field::new("prefix");
}
