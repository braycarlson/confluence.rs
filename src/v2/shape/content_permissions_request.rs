#[derive(Clone, Copy, Debug)]
pub struct ContentPermissionsRequest;

impl ContentPermissionsRequest {
    pub const PERMISSIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::OperationPermissionsInput>>,
    > = crate::Field::new("permissions");
}
