#[derive(Clone, Copy, Debug)]
pub struct Icon;

impl Icon {
    pub const HEIGHT: crate::Field<Self, crate::Integer> = crate::Field::new("height");
    pub const IS_DEFAULT: crate::Field<Self, crate::Boolean> = crate::Field::new("isDefault");
    pub const PATH: crate::Field<Self, crate::Text> = crate::Field::new("path");
    pub const WIDTH: crate::Field<Self, crate::Integer> = crate::Field::new("width");
}
