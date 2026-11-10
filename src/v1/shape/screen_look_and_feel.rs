#[derive(Clone, Copy, Debug)]
pub struct ScreenLookAndFeel;

impl ScreenLookAndFeel {
    pub const BACKGROUND: crate::Field<Self, crate::Text> = crate::Field::new("background");

    pub const BACKGROUND_ATTACHMENT: crate::Field<Self, crate::Text> =
        crate::Field::new("backgroundAttachment");

    pub const BACKGROUND_BLEND_MODE: crate::Field<Self, crate::Text> =
        crate::Field::new("backgroundBlendMode");

    pub const BACKGROUND_CLIP: crate::Field<Self, crate::Text> =
        crate::Field::new("backgroundClip");

    pub const BACKGROUND_COLOR: crate::Field<Self, crate::Text> =
        crate::Field::new("backgroundColor");

    pub const BACKGROUND_IMAGE: crate::Field<Self, crate::Text> =
        crate::Field::new("backgroundImage");

    pub const BACKGROUND_ORIGIN: crate::Field<Self, crate::Text> =
        crate::Field::new("backgroundOrigin");

    pub const BACKGROUND_POSITION: crate::Field<Self, crate::Text> =
        crate::Field::new("backgroundPosition");

    pub const BACKGROUND_REPEAT: crate::Field<Self, crate::Text> =
        crate::Field::new("backgroundRepeat");

    pub const BACKGROUND_SIZE: crate::Field<Self, crate::Text> =
        crate::Field::new("backgroundSize");

    pub const GUTTER_BOTTOM: crate::Field<Self, crate::Text> = crate::Field::new("gutterBottom");
    pub const GUTTER_LEFT: crate::Field<Self, crate::Text> = crate::Field::new("gutterLeft");
    pub const GUTTER_RIGHT: crate::Field<Self, crate::Text> = crate::Field::new("gutterRight");
    pub const GUTTER_TOP: crate::Field<Self, crate::Text> = crate::Field::new("gutterTop");

    pub const LAYER: crate::Field<Self, crate::Object<super::ScreenLookAndFeelLayer>> =
        crate::Field::new("layer");
}
