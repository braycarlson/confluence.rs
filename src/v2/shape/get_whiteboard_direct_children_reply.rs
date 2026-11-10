#[derive(Clone, Copy, Debug)]
pub struct GetWhiteboardDirectChildrenReply;

impl GetWhiteboardDirectChildrenReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::ChildrenResponse>>> =
        crate::Field::new("results");
}
