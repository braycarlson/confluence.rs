#[derive(Clone, Copy, Debug)]
pub struct GetCustomContentCommentsReply;

impl GetCustomContentCommentsReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<
        Self,
        crate::List<crate::Object<super::CustomContentCommentModel>>,
    > = crate::Field::new("results");
}
