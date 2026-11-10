#[derive(Clone, Copy, Debug)]
pub struct ContentBlueprintDraftVersion;

impl ContentBlueprintDraftVersion {
    pub const NUMBER: crate::Field<Self, crate::Integer> = crate::Field::new("number");
}
