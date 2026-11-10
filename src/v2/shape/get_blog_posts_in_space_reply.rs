#[derive(Clone, Copy, Debug)]
pub struct GetBlogPostsInSpaceReply;

impl GetBlogPostsInSpaceReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::BlogPostBulk>>> =
        crate::Field::new("results");
}
