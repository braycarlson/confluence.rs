#[derive(Clone, Copy, Debug)]
pub struct Icon;

impl Icon {
    pub const IS_DEFAULT: crate::Field<Self, crate::Boolean> = crate::Field::new("isDefault");
    pub const PATH: crate::Field<Self, crate::Text> = crate::Field::new("path");
}
