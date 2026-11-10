#[derive(Clone, Copy, Debug)]
pub struct AsyncID;

impl AsyncID {
    pub const ASYNC_ID: crate::Field<Self, crate::Text> = crate::Field::new("asyncId");
}
