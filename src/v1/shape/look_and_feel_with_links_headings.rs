#[derive(Clone, Copy, Debug)]
pub struct LookAndFeelWithLinksHeadings;

impl LookAndFeelWithLinksHeadings {
    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");
}
