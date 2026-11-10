#[derive(Clone, Copy, Debug)]
pub struct RetentionPeriod;

impl RetentionPeriod {
    pub const NUMBER: crate::Field<Self, crate::Integer> = crate::Field::new("number");
    pub const UNITS: crate::Field<Self, crate::Text> = crate::Field::new("units");
}
