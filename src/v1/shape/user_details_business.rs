#[derive(Clone, Copy, Debug)]
pub struct UserDetailsBusiness;

impl UserDetailsBusiness {
    pub const DEPARTMENT: crate::Field<Self, crate::Text> = crate::Field::new("department");
    pub const LOCATION: crate::Field<Self, crate::Text> = crate::Field::new("location");
    pub const POSITION: crate::Field<Self, crate::Text> = crate::Field::new("position");
}
