#[derive(Clone, Copy, Debug)]
pub struct TopNavigationLookAndFeel;

impl TopNavigationLookAndFeel {
    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");

    pub const HIGHLIGHT_COLOR: crate::Field<Self, crate::Text> =
        crate::Field::new("highlightColor");

    pub const HOVER_OR_FOCUS: crate::Field<
        Self,
        crate::Object<super::TopNavigationLookAndFeelHoverOrFocus>,
    > = crate::Field::new("hoverOrFocus");
}
