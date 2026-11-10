#[derive(Clone, Copy, Debug)]
pub struct GetBlogPostVersionsReply;

impl GetBlogPostVersionsReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::BlogPostVersion>>> =
        crate::Field::new("results");
}
