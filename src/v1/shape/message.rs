#[derive(Clone, Copy, Debug)]
pub struct Message;

impl Message {
    pub const ARGUMENTS: crate::Field<Self, crate::List<crate::Any>> = crate::Field::new("args");
    pub const TRANSLATION: crate::Field<Self, crate::Text> = crate::Field::new("translation");
}
