#[derive(Clone, Copy, Debug)]
pub struct MenusLookAndFeel;

impl MenusLookAndFeel {
    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");

    pub const HOVER_OR_FOCUS: crate::Field<
        Self,
        crate::Object<super::MenusLookAndFeelHoverOrFocus>,
    > = crate::Field::new("hoverOrFocus");
}
