#[derive(Clone, Copy, Debug)]
pub struct VersionRestore;

impl VersionRestore {
    pub const OPERATION_KEY: crate::Field<Self, crate::Text> = crate::Field::new("operationKey");

    pub const PARAMETERS: crate::Field<Self, crate::Object<super::VersionRestoreParams>> =
        crate::Field::new("params");
}
