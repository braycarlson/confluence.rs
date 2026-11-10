#[derive(Clone, Copy, Debug)]
pub struct GetPageFooterCommentsReply;

impl GetPageFooterCommentsReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::PageCommentModel>>> =
        crate::Field::new("results");
}
