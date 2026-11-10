#[derive(Clone, Copy, Debug)]
pub struct LookAndFeelSelection;

impl LookAndFeelSelection {
    pub const LOOK_AND_FEEL_TYPE: crate::Field<Self, crate::Text> =
        crate::Field::new("lookAndFeelType");

    pub const SPACE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("spaceKey");
}
