#[derive(Clone, Copy, Debug)]
pub struct CreateFooterCommentReplyOperations;

impl CreateFooterCommentReplyOperations {
    pub const LINKS: crate::Field<Self, crate::Object<super::OptionalFieldLinks>> =
        crate::Field::new("_links");

    pub const META: crate::Field<Self, crate::Object<super::OptionalFieldMeta>> =
        crate::Field::new("meta");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::Operation>>> =
        crate::Field::new("results");
}
