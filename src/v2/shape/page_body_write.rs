#[derive(Clone, Copy, Debug)]
pub struct PageBodyWrite;

impl PageBodyWrite {
    pub const REPRESENTATION: crate::Field<Self, crate::Text> = crate::Field::new("representation");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
