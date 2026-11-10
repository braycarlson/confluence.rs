#[derive(Clone, Copy, Debug)]
pub struct PermissionCheckResponse;

impl PermissionCheckResponse {
    pub const ERRORS: crate::Field<Self, crate::List<crate::Object<super::Message>>> =
        crate::Field::new("errors");

    pub const HAS_PERMISSION: crate::Field<Self, crate::Boolean> =
        crate::Field::new("hasPermission");

    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");
}
