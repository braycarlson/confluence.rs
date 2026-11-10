#[derive(Clone, Copy, Debug)]
pub struct Breadcrumb;

impl Breadcrumb {
    pub const LABEL: crate::Field<Self, crate::Text> = crate::Field::new("label");
    pub const SEPARATOR: crate::Field<Self, crate::Text> = crate::Field::new("separator");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
