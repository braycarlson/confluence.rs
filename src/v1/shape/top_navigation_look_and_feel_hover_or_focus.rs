#[derive(Clone, Copy, Debug)]
pub struct TopNavigationLookAndFeelHoverOrFocus;

impl TopNavigationLookAndFeelHoverOrFocus {
    pub const BACKGROUND_COLOR: crate::Field<Self, crate::Text> =
        crate::Field::new("backgroundColor");

    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");
}
