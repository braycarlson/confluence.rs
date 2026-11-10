#[derive(Clone, Copy, Debug)]
pub struct SpaceMetadata;

impl SpaceMetadata {
    pub const EXPANDABLE: crate::Field<Self, crate::Any> = crate::Field::new("_expandable");

    pub const LABELS: crate::Field<Self, crate::Object<super::LabelArray>> =
        crate::Field::new("labels");
}
