#[derive(Clone, Copy, Debug)]
pub struct ContentBlogpostRestrictions;

impl ContentBlogpostRestrictions {
    pub const EXPANDABLE: crate::Field<
        Self,
        crate::Object<super::ContentBlogpostRestrictionsExpandable>,
    > = crate::Field::new("_expandable");

    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");

    pub const READ: crate::Field<Self, crate::Object<super::ContentRestriction>> =
        crate::Field::new("read");

    pub const UPDATE: crate::Field<Self, crate::Object<super::ContentRestriction>> =
        crate::Field::new("update");
}
