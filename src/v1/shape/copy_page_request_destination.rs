#[derive(Clone, Copy, Debug)]
pub struct CopyPageRequestDestination;

impl CopyPageRequestDestination {
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
