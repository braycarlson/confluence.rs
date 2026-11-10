#[derive(Clone, Copy, Debug)]
pub struct LookAndFeel;

impl LookAndFeel {
    pub const BORDERS_AND_DIVIDERS: crate::Field<
        Self,
        crate::Object<super::LookAndFeelBordersAndDividers>,
    > = crate::Field::new("bordersAndDividers");

    pub const CONTENT: crate::Field<Self, crate::Object<super::ContentLookAndFeel>> =
        crate::Field::new("content");

    pub const HEADER: crate::Field<Self, crate::Object<super::HeaderLookAndFeel>> =
        crate::Field::new("header");

    pub const HEADINGS: crate::Field<Self, crate::Object<super::LookAndFeelHeadings>> =
        crate::Field::new("headings");

    pub const HORIZONTAL_HEADER: crate::Field<
        Self,
        crate::Object<super::HorizontalHeaderLookAndFeel>,
    > = crate::Field::new("horizontalHeader");

    pub const LINKS: crate::Field<Self, crate::Object<super::LookAndFeelLinks>> =
        crate::Field::new("links");

    pub const MENUS: crate::Field<Self, crate::Object<super::MenusLookAndFeel>> =
        crate::Field::new("menus");

    pub const SPACE_REFERENCE: crate::Field<Self, crate::Any> = crate::Field::new("spaceReference");
}
