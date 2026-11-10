#[derive(Clone, Copy, Debug)]
pub struct DatabaseNumberFormat;

impl DatabaseNumberFormat {
    pub const DECIMAL_PLACES: crate::Field<Self, crate::Integer> =
        crate::Field::new("decimalPlaces");

    pub const PREFIX: crate::Field<Self, crate::Text> = crate::Field::new("prefix");
    pub const SUFFIX: crate::Field<Self, crate::Text> = crate::Field::new("suffix");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
