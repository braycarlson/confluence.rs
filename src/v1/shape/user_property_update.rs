#[derive(Clone, Copy, Debug)]
pub struct UserPropertyUpdate;

impl UserPropertyUpdate {
    pub const VALUE: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("value");
}
