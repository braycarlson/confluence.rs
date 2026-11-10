#[derive(Clone, Copy, Debug)]
pub struct Label;

impl Label {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const LABEL: crate::Field<Self, crate::Text> = crate::Field::new("label");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const PREFIX: crate::Field<Self, crate::Text> = crate::Field::new("prefix");
}
