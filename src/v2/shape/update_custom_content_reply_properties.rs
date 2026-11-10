#[derive(Clone, Copy, Debug)]
pub struct UpdateCustomContentReplyProperties;

impl UpdateCustomContentReplyProperties {
    pub const LINKS: crate::Field<Self, crate::Object<super::OptionalFieldLinks>> =
        crate::Field::new("_links");

    pub const META: crate::Field<Self, crate::Object<super::OptionalFieldMeta>> =
        crate::Field::new("meta");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::ContentProperty>>> =
        crate::Field::new("results");
}
