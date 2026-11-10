#[derive(Clone, Copy, Debug)]
pub struct NavigationLookAndFeel;

impl NavigationLookAndFeel {
    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");

    pub const HIGHLIGHT_COLOR: crate::Field<Self, crate::Text> =
        crate::Field::new("highlightColor");

    pub const HOVER_OR_FOCUS: crate::Field<
        Self,
        crate::Object<super::NavigationLookAndFeelHoverOrFocus>,
    > = crate::Field::new("hoverOrFocus");
}
