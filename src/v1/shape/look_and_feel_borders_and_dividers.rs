#[derive(Clone, Copy, Debug)]
pub struct LookAndFeelBordersAndDividers;

impl LookAndFeelBordersAndDividers {
    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");
}
