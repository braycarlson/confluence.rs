#[derive(Clone, Copy, Debug)]
pub struct MultiEntityLinks;

impl MultiEntityLinks {
    pub const BASE: crate::Field<Self, crate::Text> = crate::Field::new("base");
    pub const NEXT: crate::Field<Self, crate::Text> = crate::Field::new("next");
}
