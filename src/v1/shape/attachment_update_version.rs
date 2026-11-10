#[derive(Clone, Copy, Debug)]
pub struct AttachmentUpdateVersion;

impl AttachmentUpdateVersion {
    pub const NUMBER: crate::Field<Self, crate::Integer> = crate::Field::new("number");
}
