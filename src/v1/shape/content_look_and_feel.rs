#[derive(Clone, Copy, Debug)]
pub struct ContentLookAndFeel;

impl ContentLookAndFeel {
    pub const BODY: crate::Field<Self, crate::Object<super::ContainerLookAndFeel>> =
        crate::Field::new("body");

    pub const CONTAINER: crate::Field<Self, crate::Object<super::ContainerLookAndFeel>> =
        crate::Field::new("container");

    pub const HEADER: crate::Field<Self, crate::Object<super::ContainerLookAndFeel>> =
        crate::Field::new("header");

    pub const SCREEN: crate::Field<Self, crate::Object<super::ScreenLookAndFeel>> =
        crate::Field::new("screen");
}
