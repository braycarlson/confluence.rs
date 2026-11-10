#[derive(Clone, Copy, Debug)]
pub struct SuperBatchWebResourcesUris;

impl SuperBatchWebResourcesUris {
    pub const ALL: crate::Field<Self, crate::Any> = crate::Field::new("all");
    pub const CSS: crate::Field<Self, crate::Any> = crate::Field::new("css");
    pub const JS_VALUE: crate::Field<Self, crate::Any> = crate::Field::new("js");
}
