#[derive(Clone, Copy, Debug)]
pub struct GetCustomContentByTypeInBlogPostReply;

impl GetCustomContentByTypeInBlogPostReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::CustomContentBulk>>> =
        crate::Field::new("results");
}
