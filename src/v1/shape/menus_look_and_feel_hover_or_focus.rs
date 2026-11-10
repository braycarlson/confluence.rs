#[derive(Clone, Copy, Debug)]
pub struct MenusLookAndFeelHoverOrFocus;

impl MenusLookAndFeelHoverOrFocus {
    pub const BACKGROUND_COLOR: crate::Field<Self, crate::Text> =
        crate::Field::new("backgroundColor");
}
