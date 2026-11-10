#[derive(Clone, Copy, Debug)]
pub struct UserWatch;

impl UserWatch {
    pub const WATCHING: crate::Field<Self, crate::Boolean> = crate::Field::new("watching");
}
