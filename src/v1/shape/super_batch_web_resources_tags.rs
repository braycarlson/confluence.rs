#[derive(Clone, Copy, Debug)]
pub struct SuperBatchWebResourcesTags;

impl SuperBatchWebResourcesTags {
    pub const ALL: crate::Field<Self, crate::Text> = crate::Field::new("all");
    pub const CSS: crate::Field<Self, crate::Text> = crate::Field::new("css");
    pub const DATA: crate::Field<Self, crate::Text> = crate::Field::new("data");
    pub const JS_VALUE: crate::Field<Self, crate::Text> = crate::Field::new("js");
}
