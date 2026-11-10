#[derive(Clone, Copy, Debug)]
pub struct LookAndFeelWithLinks;

impl LookAndFeelWithLinks {
    pub const BORDERS_AND_DIVIDERS: crate::Field<
        Self,
        crate::Object<super::LookAndFeelWithLinksBordersAndDividers>,
    > = crate::Field::new("bordersAndDividers");

    pub const CONTENT: crate::Field<Self, crate::Object<super::ContentLookAndFeel>> =
        crate::Field::new("content");

    pub const HEADER: crate::Field<Self, crate::Object<super::HeaderLookAndFeel>> =
        crate::Field::new("header");

    pub const HEADINGS: crate::Field<Self, crate::Object<super::LookAndFeelWithLinksHeadings>> =
        crate::Field::new("headings");

    pub const HORIZONTAL_HEADER: crate::Field<
        Self,
        crate::Object<super::HorizontalHeaderLookAndFeel>,
    > = crate::Field::new("horizontalHeader");

    pub const HYPERMEDIA_LINKS: crate::Field<Self, crate::Map<crate::Any>> =
        crate::Field::new("_links");

    pub const LINKS: crate::Field<Self, crate::Object<super::LookAndFeelWithLinksLinks>> =
        crate::Field::new("links");

    pub const MENUS: crate::Field<Self, crate::Object<super::MenusLookAndFeel>> =
        crate::Field::new("menus");

    pub const SPACE_REFERENCE: crate::Field<Self, crate::Any> = crate::Field::new("spaceReference");
}
