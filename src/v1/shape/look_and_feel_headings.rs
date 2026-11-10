#[derive(Clone, Copy, Debug)]
pub struct LookAndFeelHeadings;

impl LookAndFeelHeadings {
    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");
}
