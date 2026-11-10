#[derive(Clone, Copy, Debug)]
pub struct GetSpaceByIDReplyPermissions;

impl GetSpaceByIDReplyPermissions {
    pub const LINKS: crate::Field<Self, crate::Object<super::OptionalFieldLinks>> =
        crate::Field::new("_links");

    pub const META: crate::Field<Self, crate::Object<super::OptionalFieldMeta>> =
        crate::Field::new("meta");

    pub const RESULTS: crate::Field<
        Self,
        crate::List<crate::Object<super::SpacePermissionAssignment>>,
    > = crate::Field::new("results");
}
