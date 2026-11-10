#[derive(Clone, Copy, Debug)]
pub struct CreateBulkUserLookupReply;

impl CreateBulkUserLookupReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::User>>> =
        crate::Field::new("results");
}
