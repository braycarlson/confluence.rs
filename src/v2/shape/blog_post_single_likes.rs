#[derive(Clone, Copy, Debug)]
pub struct BlogPostSingleLikes;

impl BlogPostSingleLikes {
    pub const LINKS: crate::Field<Self, crate::Object<super::OptionalFieldLinks>> =
        crate::Field::new("_links");

    pub const META: crate::Field<Self, crate::Object<super::OptionalFieldMeta>> =
        crate::Field::new("meta");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::Like>>> =
        crate::Field::new("results");
}
