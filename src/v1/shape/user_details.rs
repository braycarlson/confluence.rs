#[derive(Clone, Copy, Debug)]
pub struct UserDetails;

impl UserDetails {
    pub const BUSINESS: crate::Field<Self, crate::Object<super::UserDetailsBusiness>> =
        crate::Field::new("business");

    pub const PERSONAL: crate::Field<Self, crate::Object<super::UserDetailsPersonal>> =
        crate::Field::new("personal");
}
