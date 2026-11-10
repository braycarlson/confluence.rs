#[derive(Clone, Copy, Debug)]
pub struct UserPropertyKeyArrayResults;

impl UserPropertyKeyArrayResults {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
}
