#[derive(Clone, Copy, Debug)]
pub struct ContentPropertyUpdateVersion;

impl ContentPropertyUpdateVersion {
    pub const MINOR_EDIT: crate::Field<Self, crate::Boolean> = crate::Field::new("minorEdit");
    pub const NUMBER: crate::Field<Self, crate::Any> = crate::Field::new("number");
}
