#[derive(Clone, Copy, Debug)]
pub struct GetFooterCommentsReply;

impl GetFooterCommentsReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::FooterCommentModel>>> =
        crate::Field::new("results");
}
