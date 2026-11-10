#[derive(Clone, Copy, Debug)]
pub struct GetBlogpostContentPropertiesReply;

impl GetBlogpostContentPropertiesReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::ContentProperty>>> =
        crate::Field::new("results");
}
