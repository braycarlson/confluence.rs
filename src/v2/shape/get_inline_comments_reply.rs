#[derive(Clone, Copy, Debug)]
pub struct GetInlineCommentsReply;

impl GetInlineCommentsReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::InlineCommentModel>>> =
        crate::Field::new("results");
}
