#[derive(Clone, Copy, Debug)]
pub struct BulkContentBodyConversionInput;

impl BulkContentBodyConversionInput {
    pub const CONVERSION_INPUTS: crate::Field<
        Self,
        crate::List<crate::Object<super::ContentBodyConversionInput>>,
    > = crate::Field::new("conversionInputs");
}
