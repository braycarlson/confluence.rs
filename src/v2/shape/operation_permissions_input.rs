#[derive(Clone, Copy, Debug)]
pub struct OperationPermissionsInput;

impl OperationPermissionsInput {
    pub const OPERATION: crate::Field<Self, crate::Text> = crate::Field::new("operation");

    pub const PRINCIPALS: crate::Field<
        Self,
        crate::List<crate::Object<super::ContentPrincipalInput>>,
    > = crate::Field::new("principals");
}
