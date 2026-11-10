#[derive(Clone, Copy, Debug)]
pub struct GetWhiteboardPermissionsReply;

impl GetWhiteboardPermissionsReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<
        Self,
        crate::List<crate::Object<super::ContentPermissionOutput>>,
    > = crate::Field::new("results");
}
