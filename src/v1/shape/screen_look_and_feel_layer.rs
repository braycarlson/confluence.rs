#[derive(Clone, Copy, Debug)]
pub struct ScreenLookAndFeelLayer;

impl ScreenLookAndFeelLayer {
    pub const HEIGHT: crate::Field<Self, crate::Text> = crate::Field::new("height");
    pub const WIDTH: crate::Field<Self, crate::Text> = crate::Field::new("width");
}
