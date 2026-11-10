#[derive(Clone, Copy, Debug)]
pub struct LongTaskLinks;

impl LongTaskLinks {
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
}
