#[derive(Clone, Copy, Debug)]
pub struct SpacePermission;

impl SpacePermission {
    pub const ANONYMOUS_ACCESS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("anonymousAccess");

    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");

    pub const OPERATION: crate::Field<Self, crate::Object<super::OperationCheckResult>> =
        crate::Field::new("operation");

    pub const SUBJECTS: crate::Field<Self, crate::Object<super::SpacePermissionSubjects>> =
        crate::Field::new("subjects");

    pub const UNLICENSED_ACCESS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("unlicensedAccess");
}
