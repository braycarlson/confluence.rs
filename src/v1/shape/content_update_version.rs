#[derive(Clone, Copy, Debug)]
pub struct ContentUpdateVersion;

impl ContentUpdateVersion {
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
    pub const NUMBER: crate::Field<Self, crate::Integer> = crate::Field::new("number");
}
