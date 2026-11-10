#[derive(Clone, Copy, Debug)]
pub struct LookAndFeelLinks;

impl LookAndFeelLinks {
    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");
}
