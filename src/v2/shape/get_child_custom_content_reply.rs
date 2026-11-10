#[derive(Clone, Copy, Debug)]
pub struct GetChildCustomContentReply;

impl GetChildCustomContentReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::ChildCustomContent>>> =
        crate::Field::new("results");
}
