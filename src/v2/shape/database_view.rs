#[derive(Clone, Copy, Debug)]
pub struct DatabaseView;

impl DatabaseView {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
