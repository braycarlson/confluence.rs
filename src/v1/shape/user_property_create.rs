#[derive(Clone, Copy, Debug)]
pub struct UserPropertyCreate;

impl UserPropertyCreate {
    pub const VALUE: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("value");
}
