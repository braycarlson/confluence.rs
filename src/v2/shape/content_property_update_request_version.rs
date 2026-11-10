#[derive(Clone, Copy, Debug)]
pub struct ContentPropertyUpdateRequestVersion;

impl ContentPropertyUpdateRequestVersion {
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
    pub const NUMBER: crate::Field<Self, crate::Integer> = crate::Field::new("number");
}
