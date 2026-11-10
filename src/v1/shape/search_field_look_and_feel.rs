#[derive(Clone, Copy, Debug)]
pub struct SearchFieldLookAndFeel;

impl SearchFieldLookAndFeel {
    pub const BACKGROUND_COLOR: crate::Field<Self, crate::Text> =
        crate::Field::new("backgroundColor");

    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");
}
