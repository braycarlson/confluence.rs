#[derive(Clone, Copy, Debug)]
pub struct DatabaseSelectOption;

impl DatabaseSelectOption {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const LABEL: crate::Field<Self, crate::Text> = crate::Field::new("label");
}
