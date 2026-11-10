#[derive(Clone, Copy, Debug)]
pub struct ContentBlueprintDraftSpace;

impl ContentBlueprintDraftSpace {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
}
