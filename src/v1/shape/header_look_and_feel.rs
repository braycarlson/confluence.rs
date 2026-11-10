#[derive(Clone, Copy, Debug)]
pub struct HeaderLookAndFeel;

impl HeaderLookAndFeel {
    pub const BACKGROUND_COLOR: crate::Field<Self, crate::Text> =
        crate::Field::new("backgroundColor");

    pub const BUTTON: crate::Field<Self, crate::Object<super::ButtonLookAndFeel>> =
        crate::Field::new("button");

    pub const PRIMARY_NAVIGATION: crate::Field<Self, crate::Object<super::NavigationLookAndFeel>> =
        crate::Field::new("primaryNavigation");

    pub const SEARCH: crate::Field<Self, crate::Object<super::SearchFieldLookAndFeel>> =
        crate::Field::new("search");

    pub const SECONDARY_NAVIGATION: crate::Field<
        Self,
        crate::Object<super::NavigationLookAndFeel>,
    > = crate::Field::new("secondaryNavigation");
}
