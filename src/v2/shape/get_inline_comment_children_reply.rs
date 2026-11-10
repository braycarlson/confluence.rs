#[derive(Clone, Copy, Debug)]
pub struct GetInlineCommentChildrenReply;

impl GetInlineCommentChildrenReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<
        Self,
        crate::List<crate::Object<super::InlineCommentChildrenModel>>,
    > = crate::Field::new("results");
}
