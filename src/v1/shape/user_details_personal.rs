#[derive(Clone, Copy, Debug)]
pub struct UserDetailsPersonal;

impl UserDetailsPersonal {
    pub const EMAIL: crate::Field<Self, crate::Text> = crate::Field::new("email");
    pub const IM_VALUE: crate::Field<Self, crate::Text> = crate::Field::new("im");
    pub const PHONE: crate::Field<Self, crate::Text> = crate::Field::new("phone");
    pub const WEBSITE: crate::Field<Self, crate::Text> = crate::Field::new("website");
}
