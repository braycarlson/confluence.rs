#[derive(Clone, Copy, Debug)]
pub struct ContentBlueprintDraftAncestors;

impl ContentBlueprintDraftAncestors {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
