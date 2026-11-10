#[derive(Clone, Copy, Debug)]
pub struct LookAndFeelWithLinksBordersAndDividers;

impl LookAndFeelWithLinksBordersAndDividers {
    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");
}
